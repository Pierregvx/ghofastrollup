use zk_evm_1_4_1::aux_structures::Timestamp;
use zksync_state::WriteStorage;
use zksync_types::{l2_to_l1_log::L2ToL1Log, VmEvent};

use crate::{
    interface::L1BatchEnv,
    vm_latest::{
        old_vm::{events::merge_events, history_recorder::HistoryMode},
        types::internals::ZkSyncVmState,
    },
};

pub(crate) fn collect_events_and_l1_system_logs_after_timestamp<S: WriteStorage, H: HistoryMode>(
    vm_state: &ZkSyncVmState<S, H>,
    batch_env: &L1BatchEnv,
    from_timestamp: Timestamp,
) -> (Vec<VmEvent>, Vec<L2ToL1Log>) {
    let (raw_events, l1_messages) = vm_state
        .event_sink
        .get_events_and_l2_l1_logs_after_timestamp(from_timestamp);
    let events = merge_events(raw_events)
        .into_iter()
        .map(|e| e.into_vm_event(batch_env.number))
        .collect();
    (events, l1_messages.into_iter().map(Into::into).collect())
}