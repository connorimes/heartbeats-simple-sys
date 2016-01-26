/**
 * Just tests that the functions are all there.
 */
#include <assert.h>
#include <inttypes.h>
#include <stdlib.h>
#include "heartbeat-pow.h"

int main(void) {
  uint64_t window_size = 20;
  heartbeat_pow_context hb;
  heartbeat_pow_record* window_buffer = malloc(window_size * sizeof(heartbeat_pow_record));
  assert(window_buffer);
  heartbeat_pow_init(&hb, window_size, window_buffer, -1, NULL);
  heartbeat_pow(&hb, 0, 1, 0, 1000000000, 0, 1000000);
  hb_pow_log_header(1);
  hb_pow_log_window_buffer(&hb, 1);

  hb_pow_get_window_size(&hb);
  hb_pow_get_log_fd(&hb);
  hb_pow_get_user_tag(&hb);
  hb_pow_get_global_time(&hb);
  hb_pow_get_window_time(&hb);
  hb_pow_get_global_work(&hb);
  hb_pow_get_window_work(&hb);
  hb_pow_get_global_perf(&hb);
  hb_pow_get_window_perf(&hb);
  hb_pow_get_instant_perf(&hb);
  hb_pow_get_global_energy(&hb);
  hb_pow_get_window_energy(&hb);
  hb_pow_get_global_power(&hb);
  hb_pow_get_window_power(&hb);
  hb_pow_get_instant_power(&hb);

  free(window_buffer);
  return 0;
}
