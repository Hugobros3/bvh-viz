package io.xol.badtracer

import org.joml.Vector3f

data class BBox(val min: Vector3f, val max: Vector3f) {
    inline val center: Vector3f
        get() = Vector3f(max).add(min).mul(0.5f)
    inline val radius: Vector3f
        get() = Vector3f(max).sub(min)
    inline val invRadius: Vector3f
        get() = Vector3f(radius).map { 1.0f / it }
}

inline fun Vector3f.map(functor: (Float) -> Float): Vector3f {
    x = functor(x)
    y = functor(y)
    z = functor(z)
    return this
}

inline fun Vector3f.sign() = map { if (it > 0f) 1f else if (it < 0f) -1f else 0f}

inline fun Vector3f.fold(functor: (Float, Float) -> Float) = functor(functor(x, y), z)

inline fun Vector3f.max() = fold { a, b -> kotlin.math.max(a, b) }

infix fun BBox.inside(parent: BBox) = min.x >= parent.min.x && min.y >= parent.min.y && min.z >= parent.min.z && max.x <= parent.max.x && max.y <= parent.max.y && max.z <= parent.max.z
infix fun BBox.expand(other: BBox): BBox {
    val minmin = this.min.min(other.min)
    val maxmax = this.max.max(other.max)
    return BBox(minmin, maxmax)
}

data class RayBoxIntersection(val point: Vector3f, val tMin: Float, val tMax: Float)

inline fun BBox.intersect(ray: Ray): RayBoxIntersection? {
    val minDist = 0.0
    val maxDist = 25600.0

    val origin = ray.origin
    val invDirX = ray.invDir.x()
    val invDirY = ray.invDir.y()
    val invDirZ = ray.invDir.z()

    val signDirX = invDirX < 0
    val signDirY = invDirY < 0
    val signDirZ = invDirZ < 0

    var bbox = if (signDirX) max else min
    var tmin = (bbox.x() - origin.x()) * invDirX
    bbox = if (signDirX) min else max
    var tmax = (bbox.x() - origin.x()) * invDirX
    bbox = if (signDirY) max else min
    val tymin = (bbox.y() - origin.y()) * invDirY
    bbox = if (signDirY) min else max
    val tymax = (bbox.y() - origin.y()) * invDirY

    if (tmin > tymax || tymin > tmax) {
        return null
    }
    if (tymin > tmin) {
        tmin = tymin
    }
    if (tymax < tmax) {
        tmax = tymax
    }

    bbox = if (signDirZ) max else min
    val tzmin = (bbox.z() - origin.z()) * invDirZ
    bbox = if (signDirZ) min else max
    val tzmax = (bbox.z() - origin.z()) * invDirZ

    if (tmin > tzmax || tzmin > tmax) {
        return null
    }
    if (tzmin > tmin) {
        tmin = tzmin
    }
    if (tzmax < tmax) {
        tmax = tzmax
    }
    if (tmin < maxDist && tmax > minDist) {
        val intersect = Vector3f(origin)
        val lineDirection = Vector3f(ray.direction)
        intersect.add(lineDirection.mul(tmin))
        return RayBoxIntersection(intersect, tmin, tmax)
        // return Vector3dm.add(origin,
        // lineDirection.clone().normalize().scale(tmin), null);

        // return ray.getPointAtDistance(tmin);
    }
    return null

}

inline fun BBox.doesInteresect(ray: Ray): Boolean {
    val minDist = 0.0
    val maxDist = 25600.0

    val origin = ray.origin
    val invDirX = ray.invDir.x()
    val invDirY = ray.invDir.y()
    val invDirZ = ray.invDir.z()

    val signDirX = invDirX < 0
    val signDirY = invDirY < 0
    val signDirZ = invDirZ < 0

    var bbox = if (signDirX) max else min
    var tmin = (bbox.x() - origin.x()) * invDirX
    bbox = if (signDirX) min else max
    var tmax = (bbox.x() - origin.x()) * invDirX
    bbox = if (signDirY) max else min
    val tymin = (bbox.y() - origin.y()) * invDirY
    bbox = if (signDirY) min else max
    val tymax = (bbox.y() - origin.y()) * invDirY

    if (tmin > tymax || tymin > tmax) {
        return false
    }
    if (tymin > tmin) {
        tmin = tymin
    }
    if (tymax < tmax) {
        tmax = tymax
    }

    bbox = if (signDirZ) max else min
    val tzmin = (bbox.z() - origin.z()) * invDirZ
    bbox = if (signDirZ) min else max
    val tzmax = (bbox.z() - origin.z()) * invDirZ

    if (tmin > tzmax || tzmin > tmax) {
        return false
    }
    if (tzmin > tmin) {
        tmin = tzmin
    }
    if (tzmax < tmax) {
        tmax = tzmax
    }
    if (tmin < maxDist && tmax > minDist) {
        return true
    }
    return false

}

inline fun BBox.distance(ray: Ray, minDist: Float): Float {
    val maxDist = 25600.0

    val origin = ray.origin
    val invDirX = ray.invDir.x()
    val invDirY = ray.invDir.y()
    val invDirZ = ray.invDir.z()

    val signDirX = invDirX < 0
    val signDirY = invDirY < 0
    val signDirZ = invDirZ < 0

    var bbox = if (signDirX) max else min
    var txmin = (bbox.x() - origin.x()) * invDirX
    bbox = if (signDirX) min else max
    var txmax = (bbox.x() - origin.x()) * invDirX

    bbox = if (signDirY) max else min
    val tymin = (bbox.y() - origin.y()) * invDirY
    bbox = if (signDirY) min else max
    val tymax = (bbox.y() - origin.y()) * invDirY

    if (txmin > tymax || tymin > txmax) {
        return Float.NaN
    }
    if (tymin > txmin) {
        txmin = tymin
    }
    if (tymax < txmax) {
        txmax = tymax
    }

    bbox = if (signDirZ) max else min
    val tzmin = (bbox.z() - origin.z()) * invDirZ
    bbox = if (signDirZ) min else max
    val tzmax = (bbox.z() - origin.z()) * invDirZ

    if (txmin > tzmax || tzmin > txmax) {
        return Float.NaN
    }
    if (tzmin > txmin) {
        txmin = tzmin
    }
    if (tzmax < txmax) {
        txmax = tzmax
    }
    if (txmin < maxDist && txmax > minDist) {
        return txmin
    }
    return Float.NaN

}

/*inline fun BBox.fastIntersect(ray: Ray): Float {
    val origin2 = Vector3f(ray.origin).sub(center)
    val winding = if (Vector3f(ray.origin).mul(invRadius).absolute().max() < 1.0f) -1f else 1f
    val sign = Vector3f(ray.direction).sign().negate()
    val d = Vector3f(radius).mul(winding).mul(sign).sub(ray.origin)
    <
}*/