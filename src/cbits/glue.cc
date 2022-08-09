#include "biteopt.h"
#include <cstdlib>
#include <cstring>

extern "C" {
    int biteopt_optimize(double* minimized_out,
                         int dimension,
                         void* userdata,
                         double lower_bound,
                         double upper_bound,
                         int iter,
                         int depth,
                         int attc,
                         double (*evaluate)(int, const double*, void*));
}

int biteopt_optimize(double* minimized_out,
                     int dimension,
                     void* userdata,
                     double lower_bound,
                     double upper_bound,
                     int iter,
                     int depth,
                     int attc,
                     double (*evaluate)(int, const double*, void*)) {
    if (dimension == 0) {
        return 0;
    }
    memset(minimized_out, 0, sizeof(double) * dimension);

    double minimized_value = 0.0;
    double* lb = (double*) malloc(sizeof(double) * dimension);
    double* ub = (double*) malloc(sizeof(double) * dimension);

    if (!lb || !ub) {
        free(lb);
        free(ub);
        return -1;
    }

    for (int i1 = 0; i1 < dimension; ++i1) {
        lb[i1] = lower_bound;
        ub[i1] = upper_bound;
    }

    biteopt_minimize(dimension,
                     evaluate,
                     userdata,
                     lb,
                     ub,
                     minimized_out,
                     &minimized_value,
                     iter,
                     depth,
                     attc);

    free(lb);
    free(ub);
    return 0;
}
