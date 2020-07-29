package io.xol.badtracer.debug

import io.xol.badtracer.BBox
import io.xol.badtracer.Node8
import io.xol.badtracer.inside
import io.xol.badtracer.loadRodentFormat
import org.joml.Vector3f
import java.io.File

fun main(a: Array<String>) {
    //val fis = FileInputStream("C:\\Users\\Gobrosse\\Desktop\\test-rodent\\data\\bvh.bin")
    //val fis = FileInputStream("bvh.bin")
    val (nodes, prims) = loadRodentFormat(File("C:\\Users\\Gobrosse\\AppData\\Local\\Packages\\CanonicalGroupLimited.Ubuntu18.04onWindows_79rhkp1fndgsc\\LocalState\\rootfs\\home\\hugo\\git\\anydsl2\\rodent\\build\\data\\bvh.bin"))

    //println(prims)
    val node0 = nodes[0]
    for (i in 0..7) {
        val avgPoint = (Vector3f(node0.bounds[0][i], node0.bounds[1][i], node0.bounds[2][i]).add(Vector3f(node0.bounds[0 + 3][i], node0.bounds[1 + 3][i], node0.bounds[2 + 3][i]))).mul(0.5f)
        println("$i: $avgPoint")
    }

    val dbgObj = StringBuilder()
    var gc = 1

    infix fun StringBuilder.append(s: String) = this.append(s)

    fun drawTris(pid: Int) {
        val prim = prims[pid]

        for (i in 0..3) {
            if (prim.prim_id[i] < 0)
                break

            val v0 = (0..2).map { prim.v0[it][i] }.toFloatArray()
            val e1 = (0..2).map { prim.e1[it][i] }.toFloatArray()
            val e2 = (0..2).map { prim.e2[it][i] }.toFloatArray()

            val v1 = v0.zip(e1).map { (a, b) -> a - b }.toFloatArray()
            val v2 = v0.zip(e2).map { (a, b) -> a + b }.toFloatArray()

            dbgObj append "v ${v0[0]} ${v0[1]} ${v0[2]}\n"
            dbgObj append "v ${v1[0]} ${v1[1]} ${v1[2]}\n"
            dbgObj append "v ${v2[0]} ${v2[1]} ${v2[2]}\n"

            dbgObj append "f ${gc++} ${gc++} ${gc++}\n"
        }

        if (prim.prim_id[3] != -1)
            drawTris(pid + 1)
    }

    fun drawNode(node: Node8) {
        for (child in 0..7) {
            val cn = node.child[child]
            if (cn == 0)
                break;
            if (cn > 0)
                drawNode(nodes[cn - 1])
            if (cn < 0)
                drawTris(cn xor -1)
        }
    }

    drawNode(nodes[0])

    File("reconstructed.obj").writeText(dbgObj.toString())

    var of = StringBuilder()
    var gvc = 1
    fun cube(min: Vector3f, max: Vector3f) {
        of append "v " + min.x + " " + min.y + " " + min.z + "\n"; // 0
        of append "v " + min.x + " " + min.y + " " + max.z + "\n"; // 1
        of append "v " + min.x + " " + max.y + " " + min.z + "\n"; // 2
        of append "v " + min.x + " " + max.y + " " + max.z + "\n"; // 3
        of append "v " + max.x + " " + min.y + " " + min.z + "\n"; // 4
        of append "v " + max.x + " " + min.y + " " + max.z + "\n"; // 5
        of append "v " + max.x + " " + max.y + " " + min.z + "\n"; // 6
        of append "v " + max.x + " " + max.y + " " + max.z + "\n"; // 7

        of append "f " + (gvc + 0) + " " + (gvc + 1) + " " + (gvc + 3) + " " + (gvc + 2) + "\n";
        of append "f " + (gvc + 4) + " " + (gvc + 5) + " " + (gvc + 7) + " " + (gvc + 6) + "\n";

        of append "f " + (gvc + 0) + " " + (gvc + 2) + " " + (gvc + 6) + " " + (gvc + 4) + "\n";
        of append "f " + (gvc + 1) + " " + (gvc + 3) + " " + (gvc + 7) + " " + (gvc + 5) + "\n";

        of append "f " + (gvc + 1) + " " + (gvc + 0) + " " + (gvc + 4) + " " + (gvc + 5) + "\n";
        of append "f " + (gvc + 3) + " " + (gvc + 2) + " " + (gvc + 6) + " " + (gvc + 7) + "\n";

        gvc += 8
    }

    fun nodeBBOX(i: Int, node: Node8) {

        for (child in 0..7) {
            val cn = node.child[child]
            if (cn == 0)
                break

            val min = Vector3f(node.bounds[0][child], node.bounds[2][child], node.bounds[4][child])
            val max = Vector3f(node.bounds[1][child], node.bounds[3][child], node.bounds[5][child])
            cube(min, max)

            if (cn > 0) {
                of append "o Node_$i\n"
                nodeBBOX(cn - 1, nodes[cn - 1])
            }
        }
    }

    of append "o Root\n"
    nodeBBOX(0, node0)

    File("bvh_bboxes.obj").writeText(of.toString())

    fun check(pid: Int, parent: BBox) {
        val prim = prims[pid]

        for (i in 0..3) {
            if (prim.prim_id[i] < 0)
                break

            val v0f = (0..2).map { prim.v0[it][i] }.toFloatArray()
            val e1 = (0..2).map { prim.e1[it][i] }.toFloatArray()
            val e2 = (0..2).map { prim.e2[it][i] }.toFloatArray()

            val v0 = v0f.toVec3()
            val v1 = v0f.zip(e1).map { (a, b) -> a - b }.toFloatArray().toVec3()
            val v2 = v0f.zip(e2).map { (a, b) -> a + b }.toFloatArray().toVec3()

            //if (!(v0 inside parent) || !(v1 inside parent) || !(v2 inside parent))
            //    throw RuntimeException("fail")
        }

        if (prim.prim_id[3] != -1)
            check(pid + 1, parent)
    }

    fun check(node: Node8, parent: BBox?) {
        var cc = 0
        for (child in 0..7) {
            val cn = node.child[child]
            if (cn == 0)
                break
            val slack = 0.001f // because fp sucks
            val min = Vector3f(node.bounds[0][child] - slack, node.bounds[2][child] - slack, node.bounds[4][child] - slack)
            val max = Vector3f(node.bounds[1][child] + slack, node.bounds[3][child] + slack, node.bounds[5][child] + slack)
            val bbox = BBox(min, max)

            if (parent != null && !(bbox inside parent))
                throw RuntimeException("fail")

            if (cn > 0) {
                check(nodes[cn - 1], bbox)
                cc++
            } else if (cn < 0) {
                check(cn xor -1, bbox)
                cc++
            }
        }

        println("$cc")
        if (cc == 1)
            println("fucking singleton")
    }

    check(node0, null)

    println("done")
}

fun FloatArray.toVec3() = Vector3f(this[0], this[1], this[2])

infix fun Vector3f.inside(parent: BBox) = x >= parent.min.x && y >= parent.min.y && z >= parent.min.z && x <= parent.max.x && y <= parent.max.y && z <= parent.max.z