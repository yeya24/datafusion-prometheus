# datafusion-prometheus

[Prometheus](https://prometheus.io/) as CatalogProvider for [Datafusion](https://github.com/apache/arrow-datafusion).

Run SQL against Prometheus source.

## Example

```
 ./datafusion-prometheus
+---------------+--------------------+-------------------------------------------------------------------------+------------+------------+
| table_catalog | table_schema       | table_name                                                              | table_type | definition |
+---------------+--------------------+-------------------------------------------------------------------------+------------+------------+
| prometheus    | prometheus         | prometheus_tsdb_wal_truncate_duration_seconds_sum                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_failed_configs                                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_http_response_size_bytes_sum                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_http_response_size_bytes_bucket                              | BASE TABLE |            |
| prometheus    | prometheus         | net_conntrack_listener_conn_accepted_total                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_checkpoint_deletions_failed_total                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_chunks                                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_treecache_zookeeper_failures_total                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_series_not_found_total                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_web_federation_warnings_total                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_segment_current                                     | BASE TABLE |            |
| prometheus    | prometheus         | go_gc_duration_seconds_count                                            | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_buck_hash_sys_bytes                                         | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_gc_sys_bytes                                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_samples_sum                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_rule_group_duration_seconds                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_http_request_duration_seconds_bucket                         | BASE TABLE |            |
| prometheus    | prometheus         | net_conntrack_listener_conn_closed_total                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_snapshot_replay_error_total                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_completed_pages_total                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_web_federation_errors_total                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_template_text_expansions_total                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_exemplar_series_with_exemplars_in_storage               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_config_last_reload_success_timestamp_seconds                 | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_mspan_inuse_bytes                                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_http_failures_total                                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_duration_seconds_count                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pool_reloads_total                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_max_time_seconds                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_sync_length_seconds                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_min_time                                           | BASE TABLE |            |
| prometheus    | prometheus         | go_gc_duration_seconds                                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_out_of_bound_samples_total                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compactions_failed_total                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_out_of_order_samples_total                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_remote_storage_exemplars_in_total                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_sync_length_seconds_sum                               | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_alloc_bytes_total                                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrapes_cache_flush_forced_total                      | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_isolation_low_watermark                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_kuma_fetch_duration_seconds_count                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_engine_queries_concurrent_max                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_interval_length_seconds_count                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compactions_total                                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_active_appenders                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_ready                                                        | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_kuma_fetch_failures_total                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_populating_block                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_file_scan_duration_seconds                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_linode_failures_total                                     | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_metadata_cache_bytes                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pool_reloads_failed_total                      | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_truncations_total                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_build_info                                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_retention_limit_bytes                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_sync_failed_total                                     | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_http_request_duration_seconds_count                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compactions_skipped_total                               | BASE TABLE |            |
| prometheus    | prometheus         | promhttp_metric_handler_requests_in_flight                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_kubernetes_events_total                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_page_flushes_total                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_dns_lookup_failures_total                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_size_retentions_total                                   | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_next_gc_bytes                                               | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_other_sys_bytes                                             | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_stack_inuse_bytes                                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_duration_seconds_bucket                      | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_size_bytes_sum                         | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_heap_sys_bytes                                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_checkpoint_deletions_total                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_template_text_expansion_failures_total                       | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_mcache_inuse_bytes                                          | BASE TABLE |            |
| prometheus    | prometheus         | scrape_series_added                                                     | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_consul_rpc_duration_seconds                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_symbol_table_size_bytes                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_file_scan_duration_seconds_sum                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_samples_count                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_data_replay_duration_seconds                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pool_targets                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_fsync_duration_seconds_count                        | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_consul_rpc_duration_seconds_sum                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_exemplar_max_exemplars                                  | BASE TABLE |            |
| prometheus    | prometheus         | scrape_samples_scraped                                                  | BASE TABLE |            |
| prometheus    | prometheus         | net_conntrack_dialer_conn_closed_total                                  | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_mallocs_total                                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_series_removed_total                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_samples_bucket                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_engine_query_log_failures_total                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_lowest_timestamp_seconds                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_interval_length_seconds_sum                           | BASE TABLE |            |
| prometheus    | prometheus         | go_gc_duration_seconds_sum                                              | BASE TABLE |            |
| prometheus    | prometheus         | go_goroutines                                                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_storage_blocks_bytes                                    | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_max_time                                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_exemplar_last_exemplars_timestamp_seconds               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pools_failed_total                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_file_scan_duration_seconds_count                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_range_seconds_sum                      | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_engine_query_duration_seconds_count                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_http_response_size_bytes_count                               | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_sys_bytes                                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compactions_triggered_total                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_truncations_failed_total                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_gc_duration_seconds_sum                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_truncate_duration_seconds_count                     | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_received_updates_total                                    | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_checkpoint_creations_failed_total                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_tombstone_cleanup_seconds_count                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pool_sync_total                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_range_seconds_count                    | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_heap_objects                                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_clean_start                                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_series                                             | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_heap_alloc_bytes                                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_reloads_total                                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_file_read_errors_total                                    | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_lookups_total                                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_gc_duration_seconds_count                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrapes_sample_duplicate_timestamp_total              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_size_bytes_bucket                      | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_mspan_sys_bytes                                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_consul_rpc_duration_seconds_count                         | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_heap_released_bytes                                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_rule_evaluation_duration_seconds_count                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pool_exceeded_target_limit_total               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_truncations_failed_total                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_writes_failed_total                                 | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_alloc_bytes                                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_exemplar_out_of_order_exemplars_total                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_chunks_removed_total                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_rule_group_duration_seconds_sum                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_sync_length_seconds_count                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrapes_sample_out_of_order_total                     | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_dns_lookups_total                                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pools_total                                    | BASE TABLE |            |
| prometheus    | prometheus         | scrape_duration_seconds                                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_metadata_cache_entries                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrapes_exceeded_body_size_limit_total                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_size_bytes_count                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_engine_queries                                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrapes_exemplar_out_of_order_total                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_remote_storage_samples_in_total                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_discovered_targets                                        | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_heap_idle_bytes                                             | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_heap_inuse_bytes                                            | BASE TABLE |            |
| prometheus    | prometheus         | up                                                                      | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_series_created_total                               | BASE TABLE |            |
| prometheus    | prometheus         | go_threads                                                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrapes_sample_out_of_bounds_total                    | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_frees_total                                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_rule_group_duration_seconds_count                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_exemplar_exemplars_appended_total                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_corruptions_total                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_consul_rpc_failures_total                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_kuma_fetch_duration_seconds                               | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_stack_sys_bytes                                             | BASE TABLE |            |
| prometheus    | prometheus         | scrape_samples_post_metric_relabeling                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_updates_total                                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_engine_query_duration_seconds_sum                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_notifications_queue_length                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_min_time_seconds                                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrapes_exceeded_sample_limit_total                   | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_duration_seconds_sum                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_kuma_fetch_duration_seconds_sum                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_http_requests_total                                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_kuma_fetch_skipped_updates_total                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_exemplar_exemplars_in_storage                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_interval_length_seconds                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_mmap_chunk_corruptions_total                            | BASE TABLE |            |
| prometheus    | prometheus         | promhttp_metric_handler_requests_total                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_compaction_chunk_range_seconds_bucket                   | BASE TABLE |            |
| prometheus    | prometheus         | net_conntrack_dialer_conn_attempted_total                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_isolation_high_watermark                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_vertical_compactions_total                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_notifications_dropped_total                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_api_remote_read_queries                                      | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_fsync_duration_seconds_sum                          | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_engine_query_log_enabled                                     | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_blocks_loaded                                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_remote_storage_highest_timestamp_in_seconds                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_engine_query_duration_seconds                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_tombstone_cleanup_seconds_sum                           | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_remote_storage_string_interner_zero_reference_releases_total | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_mcache_sys_bytes                                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_treecache_watcher_goroutines                                 | BASE TABLE |            |
| prometheus    | prometheus         | go_info                                                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_chunks_created_total                               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_notifications_alertmanagers_discovered                       | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_rule_evaluation_duration_seconds_sum                         | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_checkpoint_creations_total                              | BASE TABLE |            |
| prometheus    | prometheus         | net_conntrack_dialer_conn_established_total                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_http_request_duration_seconds_sum                            | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_rule_evaluation_duration_seconds                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_time_retentions_total                                   | BASE TABLE |            |
| prometheus    | prometheus         | net_conntrack_dialer_conn_failed_total                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_lowest_timestamp                                        | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_target_scrape_pool_exceeded_label_limits_total               | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_samples_appended_total                             | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_tombstone_cleanup_seconds_bucket                        | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_notifications_queue_capacity                                 | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_wal_fsync_duration_seconds                              | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_sd_azure_failures_total                                      | BASE TABLE |            |
| prometheus    | prometheus         | go_memstats_last_gc_time_seconds                                        | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_head_truncations_total                                  | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_config_last_reload_successful                                | BASE TABLE |            |
| prometheus    | prometheus         | prometheus_tsdb_reloads_failures_total                                  | BASE TABLE |            |
| prometheus    | information_schema | tables                                                                  | VIEW       |            |
| prometheus    | information_schema | columns                                                                 | VIEW       |            |
| datafusion    | information_schema | tables                                                                  | VIEW       |            |
| datafusion    | information_schema | columns                                                                 | VIEW       |            |
+---------------+--------------------+-------------------------------------------------------------------------+------------+------------+

> select * from prometheus.prometheus.prometheus_http_requests_total;
+--------------------------------+------+----------------------------+----------------+------------+----------------+-------+
| __name__                       | code | handler                    | instance       | job        | timestamp      | value |
+--------------------------------+------+----------------------------+----------------+------------+----------------+-------+
| prometheus_http_requests_total | 200  | /-/ready                   | localhost:9090 | prometheus | 1659843317.706 | 2     |
| prometheus_http_requests_total | 200  | /api/v1/label/:name/values | localhost:9090 | prometheus | 1659843317.706 | 6     |
| prometheus_http_requests_total | 200  | /api/v1/labels             | localhost:9090 | prometheus | 1659843317.706 | 5     |
| prometheus_http_requests_total | 200  | /api/v1/metadata           | localhost:9090 | prometheus | 1659843317.706 | 1     |
| prometheus_http_requests_total | 200  | /api/v1/query              | localhost:9090 | prometheus | 1659843317.706 | 39    |
| prometheus_http_requests_total | 200  | /api/v1/series             | localhost:9090 | prometheus | 1659843317.706 | 3     |
| prometheus_http_requests_total | 200  | /api/v1/status/tsdb        | localhost:9090 | prometheus | 1659843317.706 | 1     |
| prometheus_http_requests_total | 200  | /api/v1/targets            | localhost:9090 | prometheus | 1659843317.706 | 1     |
| prometheus_http_requests_total | 200  | /favicon.ico               | localhost:9090 | prometheus | 1659843317.706 | 3     |
| prometheus_http_requests_total | 200  | /graph                     | localhost:9090 | prometheus | 1659843317.706 | 2     |
| prometheus_http_requests_total | 200  | /manifest.json             | localhost:9090 | prometheus | 1659843317.706 | 2     |
| prometheus_http_requests_total | 200  | /metrics                   | localhost:9090 | prometheus | 1659843317.706 | 868   |
| prometheus_http_requests_total | 200  | /static/*filepath          | localhost:9090 | prometheus | 1659843317.706 | 9     |
| prometheus_http_requests_total | 302  | /                          | localhost:9090 | prometheus | 1659843317.706 | 2     |
| prometheus_http_requests_total | 400  | /api/v1/series             | localhost:9090 | prometheus | 1659843317.706 | 2     |
+--------------------------------+------+----------------------------+----------------+------------+----------------+-------+

> select count(*) from prometheus.prometheus.prometheus_http_requests_total;
+-----------------+
| COUNT(UInt8(1)) |
+-----------------+
| 15              |
+-----------------+

```