package io.xol.badtracer

import org.joml.Vector3fc
import java.awt.Image
import java.awt.image.BufferedImage
import java.awt.image.ImageObserver
import java.util.concurrent.Semaphore
import javax.swing.JFrame
import javax.swing.JPanel

class MyWindow(val width: Int, val height: Int) {
    var frame = 0

    lateinit var jframe: JFrame

    val image = BufferedImage(width, height, BufferedImage.TYPE_INT_ARGB)
    val input = KeyboardInput()

    var lastframe = -1L
    var fps = 0.0

    val semaphore = Semaphore(0)
    val panel = object : JPanel() {

        override fun paint(g: java.awt.Graphics?) {
            super.paint(g)

            val observer = object : ImageObserver {
                override fun imageUpdate(img: Image?, infoflags: Int, x: Int, y: Int, width: Int, height: Int): Boolean {
                    //semaphore.release()
                    return true
                }

            }
            g!!.drawImage(image, 0, 0, width, height, java.awt.Color.gray, observer)
        }
    }

    fun openWindow() {
        jframe = JFrame("DamnSimpleGraphics")

        jframe.defaultCloseOperation = JFrame.EXIT_ON_CLOSE

        jframe.contentPane = panel
        jframe.contentPane.size.setSize(width, height)
        jframe.setSize(width, height)
        //jframe.pack()

        jframe.isVisible = true
    }

    fun finishFrame() {
        jframe.repaint()
        Thread.sleep(10)
        frame++

        val delta = System.currentTimeMillis() - lastframe
        fps = 1.0 / (delta / 1000.0)
        lastframe = System.currentTimeMillis()

        jframe.title = "fps = $fps"
    }
}

typealias Color = Vector3fc

fun BufferedImage.setPixel(x: Int, y: Int, color: Color) {
    val rgb = ((color.x() * 255f).toInt().coerceIn(0, 255) shl 16) or ((color.y() * 255f).toInt().coerceIn(0, 255) shl 8) or (color.z() * 255f).toInt().coerceIn(0, 255) or (255 shl 24)
    this.setRGB(x, y, rgb)
}