#include <stdio.h>
#include <time.h>
#include <cassert>
#include <math.h>
#include <stdlib.h>
#include "vector3d.h"


// -----------------------------------------------------------------------------
// Global Constants
// -----------------------------------------------------------------------------
const double R = 1.0;

// -----------------------------------------------------------------------------
// Functions
// -----------------------------------------------------------------------------

// States how long it's been since last took call.
static void took(const char *name);

// moves v to inside the cell
vector3d fix_periodic(vector3d v, const double len);

// Return the vector pointing from a to b, assuming periodic boundaries
vector3d periodic_diff(const vector3d &a, const vector3d  &b, const double len);

// Check whether two polyhedra overlap
bool overlap(const vector3d &a, const vector3d &b, const double len);

// Move the sphere by a random amount, in a gaussian distribution with standard
// deviation dist
vector3d random_move(const vector3d &original, double dist, const double len);



int main(int argc, const char *argv[]) {
  // -----------------------------------------------------------------------------
  // Define "Constants" -- set then unchanged
  // -----------------------------------------------------------------------------
  printf("Call with %s N len iter\n  where N is the number of spheres (must be a cube),\n        len is the length of the cell sides,\n        and iter is the number of iterations to run for.\n", argv[0]);
  if (argc != 4) { return 1; }
  const int N = atoi(argv[1]);
  const float len = atof(argv[2]);
  const long iterations = atol(argv[3]);
  const double scale = 0.05;
  const double de_density = 0.01;
  char *density_fname = new char[1024];
  sprintf(density_fname, "mc/density-cpp-%i-%f-%li", N, len, iterations);

  // ---------------------------------------------------------------------------
  // Define variables
  // ---------------------------------------------------------------------------
  const int density_bins = round((3.0*len)/de_density);
  long *density_histogram = new long[density_bins]();
  vector3d *spheres = new vector3d[N]();

  // Initialize the random number generator with our seed
  random::seed(0);

  // ---------------------------------------------------------------------------
  // Set up the initial grid
  // ---------------------------------------------------------------------------
  // Balls will be initially placed on a face centered cubic (fcc) grid
  // Note that the unit cells need not be actually "cubic", but the fcc grid will
  //   be stretched to cell dimensions
  const double min_cell_width = 2*sqrt(2)*R; // minimum cell width
  const int spots_per_cell = 4; // spots in each fcc periodic unit cell
  const int cells = int(len/min_cell_width); // max number of cells per dimension
  const double cell_width = len/cells;

  // If we made our cells to small, return with error
  if(cell_width < min_cell_width) {
    printf("Placement cell size too small.");
    return 176;
  }
  // Define ball positions relative to cell position
  vector3d* offset = new vector3d[4]();
  offset[0] = vector3d(0, cell_width, cell_width)/2;
  offset[1] = vector3d(cell_width, 0, cell_width)/2;
  offset[2] = vector3d(cell_width, cell_width, 0)/2;

  // Place all balls
  int b = 0;
  for(int i = 0; i < cells; i++) {
    for(int j = 0; j < cells; j++) {
      for(int k = 0; k < cells; k++) {
        for(int l = 0; l < 4; l++) {
            spheres[b] = vector3d(i*cell_width,j*cell_width,
                                       k*cell_width) + offset[l];
            b++;
            if (b >= N) {
              goto done_placing;
            }
        }
      }
    }
  }
 done_placing:
  delete[] offset;
  took("Placement");

  // ---------------------------------------------------------------------------
  // Make sure no spheres are overlapping
  // ---------------------------------------------------------------------------
  for(int i=0; i<N; i++) {
    for(int j=i+1; j<N; j++) {
      if (overlap(spheres[i], spheres[j], len)) {
        printf("ERROR in initial placement. We have overlaps!!!\n");
        printf("AHHHHHH I DON'T KNOW WHAT TO DO!@!!!!1111\n");
        return 19;
      }
    }
  }
  fflush(stdout);

  // ---------------------------------------------------------------------------
  // MAIN PROGRAM LOOP
  // ---------------------------------------------------------------------------
  clock_t output_period = CLOCKS_PER_SEC; // start at outputting every minute
  clock_t max_output_period = clock_t(CLOCKS_PER_SEC)*60*30; // top out at half hour interval
  clock_t last_output = clock(); // when we last output data

  long totalmoves = 0;
  long workingmoves = 0;
  for(long iteration=1; iteration<=iterations; iteration++) {
    // ---------------------------------------------------------------
    // Move each sphere once
    // ---------------------------------------------------------------
    for(int i=0; i<N; i++) {
      const vector3d temp = fix_periodic(random_move(spheres[i], scale, len), len);
      bool overlaps = false;
      for(int j=0; j<N; j++) {
        if (j != i && overlap(spheres[i], spheres[j], len)) {
            overlaps = true;
            break;
          }
      }
      if (!overlaps) {
          spheres[i] = temp;
          workingmoves ++;
        }
      totalmoves ++;
    }
    // ---------------------------------------------------------------
    // Add data to density historam
    // ---------------------------------------------------------------
    for(int i=0; i<N; i++) {
      const int z_i = floor(spheres[i][2]/de_density);
      density_histogram[z_i] ++;
    }
    // ---------------------------------------------------------------
    // Save to file
    // ---------------------------------------------------------------
    const clock_t now = clock();
    if ((now - last_output > output_period) || iteration==iterations) {
      last_output = now;
      assert(last_output);
      if (output_period < max_output_period/2) output_period *= 2;
      else if (output_period < max_output_period) output_period = max_output_period;
      const double secs_done = double(now)/CLOCKS_PER_SEC;
      const int seconds = int(secs_done) % 60;
      const int minutes = int(secs_done / 60) % 60;
      const int hours = int(secs_done / 3600) % 24;
      const int days = int(secs_done / 86400);
      printf("Saving data after %i days, %02i:%02i:%02i, %li iterations complete.\n",
             days, hours, minutes, seconds, iteration);
      fflush(stdout);

      // Saving density
      FILE *densityout = fopen((const char *)density_fname, "w");
      const int zbins = round(len/de_density);
      for(int z_i = 0; z_i < zbins; z_i ++) {
        const double z = (z_i + 0.5)*de_density;
        const double zshell_volume = len*len*de_density;
        const long zhist = density_histogram[z_i];
        const double zdensity = (double)zhist*N/totalmoves/zshell_volume;
        fprintf(densityout, "%6.3f   %8.5f   %li\n", z, zdensity, zhist);
      }
      fclose(densityout);
    }
  }
  // ---------------------------------------------------------------------------
  // END OF MAIN PROGRAM LOOP
  // ---------------------------------------------------------------------------

  delete[] spheres;
  delete[] density_histogram;
  delete[] density_fname;

  return 0;
}
// -----------------------------------------------------------------------------
// END OF MAIN
// -----------------------------------------------------------------------------

static void took(const char *name) {
  assert(name); // so it'll count as being used...
  static clock_t last_time = clock();
  clock_t t = clock();
  double seconds = (t-last_time)/double(CLOCKS_PER_SEC);
  if (seconds > 120) {
    printf("%s took %.0f minutes and %g seconds.\n", name, seconds/60, fmod(seconds,60));
  } else {
    printf("%s took %g seconds..\n", name, seconds);
  }
  fflush(stdout);
  last_time = t;
}

vector3d fix_periodic(vector3d v, const double len) {
  for (int i=0; i<3; i++) {
    while (v[i] > len)
      v[i] -= len;
    while (v[i] < 0.0)
      v[i] += len;
  }
  return v;
}

vector3d periodic_diff(const vector3d &a, const vector3d  &b, const double len) {
  vector3d v = b - a;
  for (int i=0; i<3; i++) {
    while (v[i] > len/2.0)
      v[i] -= len;
    while (v[i] < -len/2.0)
      v[i] += len;
  }
  return v;
}

bool overlap(const vector3d &a, const vector3d &b, const double len) {
  const double d2 = periodic_diff(a, b, len).normsquared();
  return d2 < R*R;
}

vector3d random_move(const vector3d &original, double size, const double len) {
  vector3d temp = original;
  temp = fix_periodic(temp + vector3d::ran(size), len);
  return temp;
}
