package io.xol.badtracer

import org.joml.Vector2f
import org.joml.Vector3f
import org.joml.Vector3fc
import java.awt.event.KeyEvent
import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.tan

data class Camera(val eye: Vector3fc, val viewDirection: Vector3fc, val up: Vector3fc, val ratio: Float, val fov: Float) {
    private val theta: Float = (fov * Math.PI / 180.0).toFloat()
    private val half_height: Float = tan(theta / 2f)
    private val half_width: Float = ratio * half_height

    private val w = viewDirection
    private val u = Vector3f(up).cross(w).normalize()
    private val v = Vector3f(w).cross(u)

    //val lower_left_corner = Vector3f(-half_width, -half_height, -1.0f)
    private val lower_left_corner = Vector3f(eye).sub(Vector3f(u).mul(half_width)).sub(Vector3f(v).mul(half_height)).sub(viewDirection)

    private val horizontal = Vector3f(u).mul(half_width * 2)
    private val vertical = Vector3f(v).mul(half_height * 2)
    //val horizontal = Vector3f(2.0f * half_width, 0.0f, 0.0f)
    //val vertical = Vector3f(0.0f, 2.0f * half_height, 0.0f)

    fun makeRay(s: Float, t: Float): Ray {
        //val dir = Vector3f(lower_left_corner).add(Vector3f(horizontal).mul(s)).add(Vector3f(vertical).mul(t))
        val dir = Vector3f(lower_left_corner).add(Vector3f(horizontal).mul(s)).add(Vector3f(vertical).mul(t)).sub(eye)
        return Ray(eye, dir)
    }
}

data class Controller(val position: Vector3f, val rotation: Vector2f, val window: MyWindow) {
    var showComplexity = 0
    var anyHit = false
    var smallWindow = true
    var fast = true

    init {
        window.input.register(KeyEvent.VK_C) {
            showComplexity = (showComplexity + 1) % 3
        }
        window.input.register(KeyEvent.VK_A) {
            anyHit = !anyHit
        }
        window.input.register(KeyEvent.VK_F) {
            fast = !fast
        }
        window.input.register(KeyEvent.VK_W) {
            smallWindow = !smallWindow
        }
    }

    fun update() {
        if (window.input[KeyEvent.VK_LEFT]) {
            rotation.x += 0.125f
        } else if (window.input[KeyEvent.VK_RIGHT]) {
            rotation.x -= 0.125f
        }

        if (window.input[KeyEvent.VK_UP]) {
            rotation.y -= 0.125f
        } else if (window.input[KeyEvent.VK_DOWN]) {
            rotation.y += 0.125f
        }

        apply {
            val rotation = Vector2f(rotation)
            val viewDirection = lazy {
                val v = Vector3f(sin(rotation.x) * cos(rotation.y), sin(rotation.y), cos(rotation.x) * cos(rotation.y))
                v.mul(-0.05f)
                v
            }

            if (window.input[KeyEvent.VK_Z]) {
                position.add(viewDirection.value)
            } else if (window.input[KeyEvent.VK_S]) {
                position.sub(viewDirection.value)
            } else if (window.input[KeyEvent.VK_Q]) {
                rotation.x += Math.PI.toFloat() * 0.5f
                position.add(viewDirection.value)
            } else if (window.input[KeyEvent.VK_D]) {
                rotation.x -= Math.PI.toFloat() * 0.5f
                position.add(viewDirection.value)
            }
        }
    }

    fun toCamera(): Camera {
        val viewDirection = Vector3f(sin(rotation.x) * cos(rotation.y), sin(rotation.y), cos(rotation.x) * cos(rotation.y))
        return Camera(position, viewDirection, Vector3f(0f, 1f, 0f), window.image.width.toFloat() / window.image.height.toFloat(), 65f)
    }
}