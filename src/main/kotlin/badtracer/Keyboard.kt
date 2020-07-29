package io.xol.badtracer

import java.awt.KeyboardFocusManager
import java.awt.event.KeyEvent
import java.util.concurrent.ConcurrentHashMap

class KeyboardInput {
    private val state = ConcurrentHashMap<Int, Boolean>()
    private val events = mutableMapOf<Int, MutableList<() -> Unit>>()

    init {
        KeyboardFocusManager.getCurrentKeyboardFocusManager().addKeyEventDispatcher { keyEvent ->
            when (keyEvent.id) {
                KeyEvent.KEY_PRESSED -> {
                    state[keyEvent.keyCode] = true
                    events[keyEvent.keyCode]?.forEach { it.invoke() }
                }
                KeyEvent.KEY_RELEASED -> state[keyEvent.keyCode] = false
            }
            false
        }
    }

    operator fun get(keyCode: Int) = state[keyCode] == true

    fun register(keyCode: Int, action: () -> Unit) = events.getOrPut(keyCode) { mutableListOf() }.add(action)
}