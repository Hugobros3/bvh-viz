package io.xol.badtracer

interface Node {
    val bbox: BBox
}

sealed class NodeId {
    data class Inner(val id: Int) : NodeId()
    data class Leaf(val id: Int) : NodeId()
}

data class InnerNode(override val bbox: BBox, val children: Array<NodeId>) : Node

data class LeafNode<P>(override val bbox: BBox, val primitives: Array<P>) : Node

data class BVH<P>(val rootNode: NodeId.Inner, val innerNodes: List<InnerNode>, val leafNodes: List<LeafNode<P>>) {
    operator fun get(nodeId: NodeId) = when (nodeId) {
        is NodeId.Inner -> innerNodes[nodeId.id]
        is NodeId.Leaf -> leafNodes[nodeId.id]
    }
}