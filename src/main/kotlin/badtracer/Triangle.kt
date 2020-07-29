package io.xol.badtracer

import org.joml.Vector3f
import org.joml.Vector3fc

data class Triangle(val v0: Vector3fc, val v1: Vector3fc, val v2: Vector3fc, val e1: Vector3fc, val e2: Vector3fc)

const val EPSILON = 0.000001f

data class TriangleIntersection(val point: Vector3f, val t: Float)

fun Triangle.intersect(ray: Ray): TriangleIntersection? {
    val vertex0 = v0
    val vertex1 = v1
    val vertex2 = v2
    val edge1 = Vector3f(vertex1)
    val edge2 = Vector3f(vertex2)
    edge1.sub(vertex0)
    edge2.sub(vertex0)
    //val edge1 = e1
    //val edge2 = e2
    val a: Float
    val f: Float
    val u: Float
    val v: Float
    val h = Vector3f(ray.direction)
    h.cross(edge2)
    a = edge1.dot(h)
    if (a > -EPSILON && a < EPSILON) {
        return null    // Le rayon est parallèle au triangle.
    }
    f = 1.0f / a
    val s = Vector3f(ray.origin)
    s.sub(vertex0)
    u = f * s.dot(h)
    if (u < 0.0f || u > 1.0) {
        return null
    }
    val q = Vector3f(s)
    q.cross(edge1)
    v = f * ray.direction.dot(q)
    if (v < 0.0 || u + v > 1.0) {
        return null
    }
    // On calcule t pour savoir ou le point d'intersection se situe sur la ligne.
    val t = f * edge2.dot(q)
    if (t > EPSILON)
    // // Intersection avec le rayon
    {
        val outIntersectionPoint = Vector3f(ray.direction)
        outIntersectionPoint.mul(t)
        outIntersectionPoint.add(ray.origin)
        return TriangleIntersection(outIntersectionPoint, t)
    } else
    // On a bien une intersection de droite, mais pas de rayon.
    {
        return null
    }
}