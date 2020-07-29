package io.xol.badtracer

import org.joml.Vector2f
import org.joml.Vector3f
import org.joml.Vector3fc
import java.io.File
import java.util.*
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import kotlin.math.ln

data class Ray(val origin: Vector3fc, val direction: Vector3fc, var tMax: Float = Float.POSITIVE_INFINITY, var hitPoint: HitPoint? = null, var steps: Int = 0) {
    val invDir: Vector3fc = Vector3f(1.0f / direction.x(), 1.0f / direction.y(), 1.0f / direction.z())
}

data class HitPoint(val position: Vector3fc)

fun main(args: Array<String>) {
    val bvhFile = File("C:\\Users\\Gobrosse\\AppData\\Local\\Packages\\CanonicalGroupLimited.Ubuntu18.04onWindows_79rhkp1fndgsc\\LocalState\\rootfs\\home\\hugo\\git\\anydsl2\\rodent\\build\\corn\\data\\bvh.bin")
    //val bvhFile = File("C:\\Users\\Gobrosse\\AppData\\Local\\Packages\\CanonicalGroupLimited.Ubuntu18.04onWindows_79rhkp1fndgsc\\LocalState\\rootfs\\home\\hugo\\git\\anydsl2\\rodent\\build\\data\\bvh.bin")
    val bvhFastFile = File("C:\\Users\\Gobrosse\\AppData\\Local\\Packages\\CanonicalGroupLimited.Ubuntu18.04onWindows_79rhkp1fndgsc\\LocalState\\rootfs\\home\\hugo\\git\\anydsl2\\rodent\\build3\\data\\bvh.bin")
    val bvh = loadBVH(bvhFile)
    val bvhFast = loadBVH(bvhFastFile)

    //println(bvh)

    val window = MyWindow(640, 480)
    window.openWindow()

    val heatmap = Heatmap()
    val controller = Controller(Vector3f(), Vector2f(), window)

    while (true) {
        controller.update()
        val camera = controller.toCamera()

        val nthreads = Runtime.getRuntime().availableProcessors()
        val exec = Executors.newFixedThreadPool(nthreads)

        var width = window.image.width
        var height = window.image.height
        if (controller.smallWindow) {
            width /= 8
            height /= 8
        }

        val bvh = if(controller.fast) bvhFast else bvh

        for (x in 0 until width) {
            exec.submit {
                for (y in 0 until height) {
                    window.image.setPixel(x, y, Vector3f(0f, 0f, 1f))

                    val u = x.toFloat() / width.toFloat()
                    val v = 1.0f - y.toFloat() / height.toFloat()

                    val ray = camera.makeRay(u, v)

                    if (bvh.trace(ray, controller.anyHit)) {
                        when (controller.showComplexity) {
                            1 -> window.image.setPixel(x, y, Vector3f(0.01525f * ln(ray.steps * 1.0).toFloat(), 0.0051525f * (ray.steps * 1.0).toFloat(), 0.0f * 0.5f * ln(ray.tMax.toDouble()).toFloat()))
                            2 -> window.image.setPixel(x, y, heatmap.colorFor(ray.steps))
                            else -> window.image.setPixel(x, y, Vector3f(0.25f * ln(ray.tMax)))
                        }
                    } else
                        window.image.setPixel(x, y, Vector3f(0.125f * ray.steps, 0.125f * 0.125f * ray.steps, 0.125f * 0.125f * 0.125f * ray.steps))
                }
            }
        }

        exec.shutdown()
        exec.awaitTermination(1000, TimeUnit.SECONDS)

        /* for (x in 0 until window.image.width) {
            for (y in 0 until window.image.height) {
                window.image.setPixel(x, y, Vector3f(0f, 0f, 1f))

                val u = x.toFloat() / window.image.width.toFloat()
                val v = 1.0f - y.toFloat() / window.image.height.toFloat()

                val ray = camera.makeRay(u, v)

                if (bvh.trace(ray, controller.anyHit)) {
                    if (controller.showComplexity)
                        window.image.setPixel(x, y, Vector3f(0.01525f * (ray.steps * 1.0).toFloat(), 0.0051525f * (ray.steps * 1.0).toFloat(), 0.0f * 0.5f * ln(ray.t.toDouble()).toFloat()))
                    else
                        window.image.setPixel(x, y, Vector3f(0.5f * ln(ray.t.toFloat())))
                } else
                    window.image.setPixel(x, y, Vector3f(0.125f * ray.steps, 0.125f * 0.125f * ray.steps, 0.125f * 0.125f * 0.125f * ray.steps))
            }
        }*/
        window.finishFrame()
    }
}

data class StackElem(val node: NodeId, val tmin: Float)

fun BVH<Triangle>.trace(ray: Ray, anyHit: Boolean): Boolean {
    //val stack = arrayListOf(StackElem(rootNode, 0f))
    val stack = PriorityQueue<StackElem> { a, b ->
        a.tmin.compareTo(b.tmin) * 10 //+ if(a.node != b.node) 1 else 0
    }
    stack.add(StackElem(rootNode, 0f))

    var closestHit = Float.MAX_VALUE
    val sorted = ArrayList<Pair<Float, NodeId>>()

    while (stack.isNotEmpty()) {
        ray.steps++

        val elem = stack.remove()
        //val elem = stack.first()
        //stack.remove(elem)
        //val elem = stack.removeAt(stack.size - 1)
        if (elem.tmin > ray.tMax)
            continue

        val node = this[elem.node]
        //val bboxIntersection = node.bbox.intersect(ray)

        //if (bboxIntersection != null) {
        // We already have a closer hit than this entire bbox ? skip it
        //if (closestHit < bboxIntersection)
        //    continue
        //ray.t = maxOf(ray.t, bboxIntersection.second)
        when (node) {
            is InnerNode -> {
                /*if (anyHit) {
                    for (child in node.children) {
                        val bboxIntersection = this[child].bbox.intersect(ray)
                        if(bboxIntersection != null)
                            stack.add(child)
                    }
                } else {*/

                for (child in node.children) {
                    val childNode = this[child]
                    val childBBox = childNode.bbox
                    //val center = Vector3f(childBBox.min).add(childBBox.max).mul(0.5f)
                    //val distance = center.distanceSquared(ray.origin)
                    val intersection = childBBox.intersect(ray) ?: continue
                    val distance = intersection.tMin

                    stack.add(StackElem(child, distance))
                }

                /*for (child in node.children) {
                    val childNode = this[child]
                    val childBBox = childNode.bbox
                    //val center = Vector3f(childBBox.min).add(childBBox.max).mul(0.5f)
                    //val distance = center.distanceSquared(ray.origin)
                    val intersection = childBBox.intersect(ray) ?: continue
                    val distance = intersection.tMin

                    var insert = 0
                    for (s in sorted) {
                        if (distance > s.first)
                            break
                        insert++
                    }
                    sorted.add(insert, Pair(distance, child))
                }

                /*while (sorted.isNotEmpty()) {
                    stack.add(sorted.removeLast().second)
                }*/
                for (s in sorted)
                    stack.add(StackElem(s.second, s.first))
                sorted.clear()*/
                //}
            }
            is LeafNode<*> -> {
                for (primitive in node.primitives) {
                    val triangle = primitive as Triangle
                    val intersection = triangle.intersect(ray)
                    if (intersection != null) {
                        if (anyHit) {
                            ray.hitPoint = HitPoint(intersection.point)
                            ray.tMax = intersection.t
                            return true
                        } else {
                            if (intersection.t < closestHit) {
                                closestHit = intersection.t
                                ray.hitPoint = HitPoint(intersection.point)
                                ray.tMax = intersection.t
                            }
                        }
                    }
                }
                //ray.hitPoint = HitPoint(Vector3f(ray.direction).mul(bboxIntersection).add(ray.origin))
                //return true
            }
            else -> throw Exception()
        }
        //}

    }

    return closestHit != Float.MAX_VALUE
}

fun hitSphere(center: Vector3fc, radius: Float, ray: Ray): Boolean {
    val oc = Vector3f(ray.origin).sub(center)
    val a = ray.direction.dot(ray.direction)
    val b = 2.0f * oc.dot(ray.direction)
    val c = oc.dot(oc) - radius * radius
    val discriminant = b * b - 4 * a * c
    return discriminant > 0
}