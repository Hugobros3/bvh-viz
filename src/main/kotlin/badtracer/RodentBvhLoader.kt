package io.xol.badtracer

import net.jpountz.lz4.LZ4Factory
import org.joml.Vector3f
import org.joml.Vector3fc
import java.io.File
import java.io.FileInputStream
import java.io.FileOutputStream
import java.nio.ByteBuffer
import java.nio.ByteOrder

data class Node8(val bounds: Array<FloatArray>, val child: IntArray, val pad: IntArray)

data class Tri4(val v0: Array<FloatArray>, val e1: Array<FloatArray>, val e2: Array<FloatArray>, val n: Array<FloatArray>, val prim_id: IntArray, val geometry_id: IntArray)

fun loadRodentFormat(file: File): Pair<List<Node8>, List<Tri4>> {
    val fis = FileInputStream(file)

    val contents = fis.readBytes()
    val input = ByteBuffer.allocate(contents.size)
    input.put(contents)
    input.flip()

    input.order(ByteOrder.LITTLE_ENDIAN)

    val fos = FileOutputStream("bvh_nodes.bin")

    while (input.remaining() > 0) {
        val node_struct_size = input.getInt()
        val prim_struct_size = input.getInt()
        println("found section $node_struct_size:$prim_struct_size")

        val nodes_buffer = readBuffer(input)
        val prim_buffer = readBuffer(input)

        fun write(byteBuffer: ByteBuffer) {
            val arr = ByteArray(byteBuffer.capacity())
            byteBuffer.get(arr, 0, byteBuffer.capacity())
            fos.write(arr)
        }

        //write(nodes_buffer)
        write(prim_buffer)


        nodes_buffer.position(0)
        nodes_buffer.limit(nodes_buffer.capacity())
        nodes_buffer.order(ByteOrder.LITTLE_ENDIAN)

        val nodes = mutableListOf<Node8>()
        for (i in 0 until nodes_buffer.capacity() / node_struct_size) {
            val bounds = (0..5).map { (0..7).map { nodes_buffer.float }.toFloatArray() }.toTypedArray()
            val children = (0..7).map { nodes_buffer.int }.toIntArray()
            val pad = (0..7).map { nodes_buffer.int }.toIntArray()
            nodes.add(Node8(bounds, children, pad))
        }

        prim_buffer.position(0)
        prim_buffer.limit(prim_buffer.capacity())
        prim_buffer.order(ByteOrder.LITTLE_ENDIAN)

        val prims = mutableListOf<Tri4>()
        for (i in 0 until prim_buffer.capacity() / prim_struct_size) {
            val v0 = (0..2).map { (0..3).map { prim_buffer.float }.toFloatArray() }.toTypedArray()
            val e1 = (0..2).map { (0..3).map { prim_buffer.float }.toFloatArray() }.toTypedArray()
            val e2 = (0..2).map { (0..3).map { prim_buffer.float }.toFloatArray() }.toTypedArray()
            val n = (0..2).map { (0..3).map { prim_buffer.float }.toFloatArray() }.toTypedArray()

            val prim_id = (0..3).map { prim_buffer.int }.toIntArray()
            val geom_id = (0..3).map { prim_buffer.int }.toIntArray()

            prims.add(Tri4(v0, e1, e2, n, prim_id, geom_id))
        }

        return Pair(nodes, prims)
    }
    throw Exception("Didn't find the BVH84")
}


fun loadBVH(file: File): BVH<Triangle> {
    val (node8s, tri4s) = loadRodentFormat(file)
    val innerNodes = mutableListOf<InnerNode>()
    val leafNodes = mutableListOf<LeafNode<Triangle>>()

    fun writeLeafNode(bbox: BBox, tri4: Tri4): NodeId.Leaf {
        val triangles = mutableListOf<Triangle>()

        for (i in 0..3) {
            val primId = tri4.prim_id[i]
            val last = primId < 0

            val v0 = tri4.v0.extractVec3(i)
            val e1 = tri4.e1.extractVec3(i)
            val e2 = tri4.e2.extractVec3(i)

            val v1 = Vector3f(v0).sub(e1)
            val v2 = Vector3f(e2).add(v0)

            val triangle = Triangle(v0, v1, v2, e1, e2)
            triangles.add(triangle)

            if (last)
                break
        }

        val node = LeafNode(bbox, triangles.toTypedArray())
        leafNodes.add(node)
        return NodeId.Leaf(leafNodes.size - 1)
    }

    fun writeInnerNode(node8: Node8): NodeId.Inner {
        var bbox = node8.extractBBox(0)
        val children = mutableListOf<NodeId>()

        for (i in 0..7) {
            val child = node8.child[i]
            if (child == 0)
                break

            val childBBox = node8.extractBBox(i)
            bbox = bbox.expand(childBBox)

            val childRef = if (child > 0) {
                val childNode8Id = child - 1
                writeInnerNode(node8s.get(childNode8Id))
            } else {
                val childTri4Id = child xor -1
                writeLeafNode(childBBox, tri4s.get(childTri4Id))
            }

            children.add(childRef)
        }

        val node = InnerNode(bbox, children.toTypedArray())
        innerNodes.add(node)
        val id = innerNodes.size - 1
        return NodeId.Inner(id)
    }

    val rootNodeId = writeInnerNode(node8s[0])

    return BVH(rootNodeId, innerNodes, leafNodes)
}

fun Array<FloatArray>.extractVec3(i: Int): Vector3fc {
    return Vector3f(this[0][i], this[1][i], this[2][i])
}

fun Node8.extractBBox(child: Int): BBox {
    return BBox(Vector3f(
            this.bounds[0][child],
            this.bounds[2][child],
            this.bounds[4][child]
    ),
            Vector3f(
                    this.bounds[1][child],
                    this.bounds[3][child],
                    this.bounds[5][child]
            )
    )
}

val decompressor = LZ4Factory.fastestInstance().fastDecompressor()
fun readBuffer(input: ByteBuffer): ByteBuffer {
    val uncompressed_size = input.getInt()
    val compressed_size = input.getInt()

    val compressed = ByteBuffer.allocateDirect(compressed_size)
    val uncompressed = ByteBuffer.allocateDirect(uncompressed_size)

    val compressedArray = ByteArray(compressed_size)
    input.get(compressedArray)
    //dis.readFully(compressedArray)
    compressed.put(compressedArray)
    compressed.flip()

    println(compressed[0])
    println("decompressing buffer $uncompressed_size:$compressed_size")

    decompressor.decompress(compressed, uncompressed)
    uncompressed.flip()
    return uncompressed
}