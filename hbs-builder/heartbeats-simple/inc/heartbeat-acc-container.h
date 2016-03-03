/**
 * Container for a heartbeat and its window buffer, with utility functions for
 * memory management at initialization and completion.
 *
 * This version is for heartbeat-acc.h.
 *
 * @author Connor Imes
 */
#ifndef _HEARTBEAT_ACC_CONTAINER_H
#define _HEARTBEAT_ACC_CONTAINER_H

#ifdef __cplusplus
extern "C" {
#endif

#include <inttypes.h>
#include "heartbeat-acc.h"

typedef struct heartbeat_acc_container {
  heartbeat_acc_context hb;
  heartbeat_acc_record* window_buffer;
} heartbeat_acc_container;

/**
 * Allocate the window buffer.
 * Only fails if hc is NULL, window_size is 0, or the window buffer cannot be
 * allocated, in which cases errno is set.
 *
 * @param hc
 * @param window_size
 * @return 0 on success, another value otherwise
 */
int heartbeat_acc_container_init(heartbeat_acc_container* hc,
                                 uint64_t window_size);

/**
 * Convenience function to initialize the container and the heartbeat context.
 * Only fails if hc is NULL, window_size is 0, or the window buffer cannot be
 * allocated, in which cases errno is set.
 *
 * @param hc
 * @param window_size
 * @param log_fd
 * @param hwc_callback
 * @return 0 on success, another value otherwise
 */
int heartbeat_acc_container_init_context(heartbeat_acc_container* hc,
                                         uint64_t window_size,
                                         int log_fd,
                                         heartbeat_acc_window_complete* hwc_callback);

/**
 * Free the window buffer.
 *
 * @param hc
 */
void heartbeat_acc_container_finish(heartbeat_acc_container* hc);

#ifdef __cplusplus
}
#endif

#endif
