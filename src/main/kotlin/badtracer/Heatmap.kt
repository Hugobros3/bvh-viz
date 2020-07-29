package io.xol.badtracer

import org.joml.Vector3f
import org.joml.Vector3fc

class Heatmap {
    fun rgb(r: Int, g: Int, b: Int) = Vector3f(r / 256.0f, g / 256.0f, b / 256.0f)

    val colors = listOf(
            rgb(87, 0, 127) to 0,
            rgb(68, 0, 206) to 8,
            rgb(0 , 76, 255) to 16,
            rgb(0, 165, 255) to 32,
            rgb(0, 255, 182) to 48,
            rgb(182, 255, 0) to 64,
            rgb(255, 0, 0) to 128
    )

    fun colorFor(steps: Int): Vector3fc {
        val matchB = colors.find { it.second >= steps} ?: colors.last()
        if(matchB.second == steps || matchB.second == 0 || steps > matchB.second)
            return matchB.first

        val matchA = colors[colors.indexOf(matchB) - 1]
        val delta = matchB.second - matchA.second
        val dist = steps - matchA.second
        val lerp = dist.toFloat() / delta.toFloat()
        val olerp = 1f - lerp
        val colorA = matchA.first
        val colorB = matchB.first
        val acc = Vector3f(colorB.x * lerp + colorA.x * (olerp),colorB.y * lerp + colorA.y * (olerp),colorB.z * lerp + colorA.z * (olerp))
        return acc
    }
}