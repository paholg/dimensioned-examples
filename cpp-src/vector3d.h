#include <math.h>
#include "MersenneTwister.h"
#include <cassert>

#pragma once

struct random {
private:
  static long unsigned int x;
public:
  static void seed(unsigned long seedval) {
    x = seedval;
  }
  static double ran() {
    static MTRand my_mtrand(x);
    return my_mtrand.randExc();
    // fixme: make this real
    return 0.7;
  }
};

class vector3d {
 public:
  double x;
  double y;
  double z;

  vector3d() {
    x = y = z = 0; }
  vector3d(const double newx, const double newy, const double newz) {
    x = newx; y = newy; z = newz; }
  vector3d(const vector3d &v) {
    x = v.x; y = v.y; z = v.z;}

  vector3d operator=(const vector3d &v) {
    x = v.x; y = v.y; z = v.z;
    return *this; }

  vector3d operator-() const {
    return vector3d(-x, -y, -z);
  }

  vector3d operator+(const vector3d &v) const {
    return vector3d(x+v.x, y+v.y, z+v.z); }
  vector3d operator+=(const vector3d &v) {
    x += v.x; y += v.y; z += v.z;
    return *this; }

  vector3d operator-(const vector3d &v) const {
    return vector3d(x-v.x, y-v.y, z-v.z); }
  vector3d operator-=(const vector3d &v) {
    x -= v.x; y -= v.y; z -= v.z;
    return *this; }

  vector3d operator*(const double scalar) const {
    return vector3d(scalar*x, scalar*y, scalar*z); }
  vector3d operator*=(const double scalar) {
    x *= scalar; y *= scalar; z *= scalar;
    return *this; }

  vector3d operator/(const double scalar) const {
    return vector3d(x/scalar, y/scalar, z/scalar); }
  vector3d operator/=(const double scalar) {
    x /= scalar; y /= scalar; z /= scalar;
    return *this; }

  bool operator ==(const vector3d &v) const {
    return ((x == v.x) && (y == v.y) &&
            (z == v.z)); }
  bool operator !=(const vector3d &v) const {
    return !(*this == v); }

  double &operator[](const unsigned int i) {
    switch(i) {
    case 0: return x;
    case 1: return y;
    case 2: return z;
    }
    assert(0);
  }
  const double operator[](const unsigned int i) const {
    switch(i) {
    case 0: return x;
    case 1: return y;
    case 2: return z;
    }
    assert(0);
  }

  double dot(const vector3d &v) const {
    return x*v.x + y*v.y + z*v.z; }
  vector3d cross(const vector3d &v) const {
    return vector3d(y*v.z - z*v.y, x*v.z - z*v.x, x*v.y - y*v.z); }

  double norm() const {
    return sqrt(x*x + y*y + z*z); }
  double normsquared() const {
    return x*x + y*y + z*z; }
  vector3d normalized() const {
    return *this/this->norm(); }

  // void tostr(char str[]) const {
  //   sprintf(str, "(%6.2f, %6.2f, %6.2f)", x, y, z);
  // }

  static vector3d ran(double scale) {
    double x, y, r2;
    do {
      x = 2*random::ran() - 1;
      y = 2*random::ran() - 1;
      r2 = x*x + y*y;
    } while(r2 >= 1 || r2 == 0);
    double fac = scale*sqrt(-2*log(r2)/r2);
    vector3d out(x*fac, y*fac, 0);
    do {
      x = 2*random::ran() - 1;
      y = 2*random::ran() - 1;
      r2 = x*x + y*y;
    } while(r2 >= 1 || r2 == 0);
    fac = scale*sqrt(-2*log(r2)/r2);
    out[2]=x*fac;
    return out;
  }
};

inline vector3d operator*(const double scalar, const vector3d &v) {
  return v*scalar; }
