package io.xol.badtracer.debug

import io.xol.badtracer.readBuffer
import java.io.File
import java.io.FileInputStream
import java.nio.ByteBuffer
import java.nio.ByteOrder

fun main(a: Array<String>) {
    val fis = FileInputStream("vertices.bin")
    //val fis = FileInputStream("C:\\Users\\Gobrosse\\Desktop\\test-rodent\\data\\vertices.bin")
    val contents = fis.readBytes()
    val input = ByteBuffer.allocate(contents.size)
    input.put(contents)
    input.flip()
    input.order(ByteOrder.LITTLE_ENDIAN)

    val tris = readBuffer(input)
    tris.order(ByteOrder.LITTLE_ENDIAN)
    tris.position(0)
    println(tris)

    val fis2 = FileInputStream("indices.bin")
    //val fis2 = FileInputStream("C:\\Users\\Gobrosse\\Desktop\\test-rodent\\data\\indices.bin")
    val contents2 = fis2.readBytes()
    val input2 = ByteBuffer.allocate(contents2.size)
    input2.put(contents2)
    input2.flip()
    input2.order(ByteOrder.LITTLE_ENDIAN)

    val indices = readBuffer(input2)
    indices.order(ByteOrder.LITTLE_ENDIAN)
    indices.position(0)
    println(indices)

    var testObj = "#test\n"

    for(i in 0 until (tris.limit() / 4) / 3) {
        val x = tris.float
        val y = tris.float
        val z = tris.float
        testObj += "v $x $y $z\n"
    }
    for(i in 0 until ((indices.limit() / 4) / 4)) {
        val i0 = indices.int + 1
        val i1 = indices.int + 1
        val i2 = indices.int + 1
        val dc = indices.int
        testObj += "f $i0 $i1 $i2\n"
    }

    val outputFile = File("out.obj")
    outputFile.writeText(testObj)
}