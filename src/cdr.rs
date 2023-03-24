use num_derive::FromPrimitive;
use num_traits::{FromPrimitive, ToPrimitive};
use polars::prelude::*;
use rand::Rng;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
struct CellRecord {
    cell_id: u32,
    latitude: f32,
    longitude: f32,
}

#[derive(Debug)]
pub struct CallRecord {
    index: u32,
    party1: &'static str,
    party2: &'static str,
    duration: u32,
    timestamp: u32,
    cell_id: u32,
}

impl CallRecord {
    pub fn mocked() -> CallRecord {
        CallRecord {
            index: rand::random(),
            party1: "",
            party2: "",
            duration: rand::random(),
            timestamp: rand::random(),
            cell_id: rand::random(),
        }
    }
}

#[derive(Debug)]
struct MessageRecord {
    party1: &'static str,
    party2: &'static str,
    timestamp: u32,
    length: u32,
}

#[derive(Debug)]
pub struct RecordRepository {
    cells: Vec<CellRecord>,
    pub calls: Vec<CallRecord>,
    messages: Vec<MessageRecord>,
}

impl RecordRepository {
    pub fn new() -> RecordRepository {
        RecordRepository {
            cells: Vec::new(),
            calls: Vec::new(),
            messages: Vec::new(),
        }
    }

    fn add_cell(&mut self, cell: CellRecord) {
        self.cells.push(cell);
    }

    pub fn add_call(&mut self, call: CallRecord) {
        self.calls.push(call);
    }

    fn add_message(&mut self, message: MessageRecord) {
        self.messages.push(message);
    }

    fn to_df(&mut self) -> DataFrame {
        let mut df = DataFrame::default();
        return df;
    }

    fn get_records(
        &mut self,
        user1: &str,
        user2: Option<&str>,
    ) -> (Vec<&CallRecord>, Vec<&MessageRecord>) {
        let mut calls = Vec::new();
        let mut messages = Vec::new();
        for call in &self.calls {
            if call.party1 == user1 || call.party2 == user1 {
                if let Some(user2) = user2 {
                    if call.party1 == user2 || call.party2 == user2 {
                        calls.push(call);
                    }
                } else {
                    calls.push(call);
                }
            }
        }
        return (calls, messages);
    }

    fn get_all_users(&mut self) -> HashSet<&&str> {
        let mut users = HashSet::new();
        for call in &self.calls {
            users.insert(&call.party1);
            users.insert(&call.party2);
        }
        for message in &self.messages {
            users.insert(&message.party1);
            users.insert(&message.party2);
        }
        return users;
    }

    fn get_peers_for_user(&mut self, user: String) -> HashSet<&&str> {
        let mut peers = HashSet::new();
        for call in &self.calls {
            if call.party1 == user {
                peers.insert(&call.party2);
            } else if call.party2 == user {
                peers.insert(&call.party1);
            }
        }
        for message in &self.messages {
            if message.party1 == user {
                peers.insert(&message.party2);
            } else if message.party2 == user {
                peers.insert(&message.party1);
            }
        }
        return peers;
    }

    fn get_connection_matrix(&mut self) -> DataFrame {
        let mut df = DataFrame::default();
        // let users = self.get_all_users();
        // let mut matrix = Vec::new();
        // for user1 in users {
        //     let mut row = Vec::new();
        //     for user2 in users {
        //         let (calls, messages) = self.get_records(user1, Some(user2));
        //         row.push(calls.len() + messages.len());
        //     }
        //     matrix.push(row);
        // }
        // df.add_column(Series::new("user", users));
        // for i in 0..users.len() {
        //     df.add_column(Series::new(users[i].clone(), matrix[i].clone()));
        // }
        return df;
    }

    fn get_edges(&mut self) -> DataFrame {
        let mut df = DataFrame::default();
        let users = self.get_all_users();
        //let mut edges = Vec::new();
        // for user1 in users {
        //     let peers = self.get_peers_for_user(user1);
        //     for user2 in peers {
        //         let (calls, messages) = self.get_records(user1, Some(user2));
        //         edges.push((user1.clone(), user2.clone(), calls.len() + messages.len()));
        //     }
        // }
        // df.add_column(Series::new("user1", edges.iter().map(|x| x.0)));
        // df.add_column(Series::new("user2", edges.iter().map(|x| x.1)));
        // df.add_column(Series::new("weight", edges.iter().map(|x| x.2)));
        return df;
    }
}

impl RecordRepository {
    pub fn mocked(num_records: u32) -> Self {
        let mut repo = RecordRepository {
            cells: Vec::new(),
            calls: Vec::new(),
            messages: Vec::new(),
        };
        for i in 0..num_records {
            repo.add_call(CallRecord {
                index: i,
                party1: "",
                party2: "",
                duration: i,
                timestamp: i,
                cell_id: i,
            });
        }
        return repo;
    }
}

// ------------------------------ Structs ------------------------------

// https://www.cisco.com/c/en/us/td/docs/voice_ip_comm/cucm/service/10_0_1/cdrdef/CUCM_BK_CBB143DE_00_cucm-cdr-administration-guide-100/CUCM_BK_CBB143DE_00_cucm-cdr-administration-guide-100_chapter_0101.html?bookSearch=true

/// Describes the reason for the call termination.
#[derive(FromPrimitive)]
enum TerminationCode {
    NoError = 0,
    UnallocatedNumber = 1,
    NoRouteToSpecifiedTransitNetwork = 2,
    NoRouteToDestination = 3,
    SendSpecialInformationTone = 4,
    MisdialledTrunkPrefix = 5,
    ChannelUnacceptable = 6,
    CallAwardedAndBeingDeliveredInAnEstablishedChannel = 7,
    Preemption = 8,
    PreemptionCircuitReservedForReuse = 9,
    NormalCallClearing = 16,
    UserBusy = 17,
    NoUserResponding = 18,
    NoAnswerFromUser = 19,
    SubscriberAbsent = 20,
    CallRejected = 21,
    NumberChanged = 22,
    NonSelectedUserClearing = 26,
    DestinationOutOfOrder = 27,
    InvalidNumberFormat = 28,
    FacilityRejected = 29,
    ResponseToStatusEnquiry = 30,
    NormalUnspecified = 31,
    NoCircuitChannelAvailable = 34,
    NetworkOutOfOrder = 38,
    PermanentFrameModeConnectionOutOfService = 39,
    PermanentFrameModeConnectionOperational = 40,
    TemporaryFailure = 41,
    SwitchingEquipmentCongestion = 42,
    AccessInformationDiscarded = 43,
    RequestedCircuitChannelNotAvailable = 44,
    PrecedenceCallBlocked = 46,
    ResourceUnavailableUnspecified = 47,
    QualityOfServiceUnavailable = 49,
    RequestedFacilityNotSubscribed = 50,
    ServiceOperationViolated = 53,
    IncomingCallsBarred = 54,
    IncomingCallsBarredWithinCUG = 55,
    BearerCapabilityNotAuthorized = 57,
    BearerCapabilityNotPresentlyAvailable = 58,
    InconsistencyInDesignatedOutgoingAccessInformationAndSubscriberClass = 62,
    ServiceOrOptionNotAvailableUnspecified = 63,
    BearerCapabilityNotImplemented = 65,
    ChannelTypeNotImplemented = 66,
    RequestedFacilityNotImplemented = 69,
    OnlyRestrictedDigitalInformationBearerCapabilityIsAvailable = 70,
    ServiceOrOptionNotImplementedUnspecified = 79,
    InvalidCallReferenceValue = 81,
    IdentifiedChannelDoesNotExist = 82,
    ASuspendedCallExistsButThisCallIdentityDoesNot = 83,
    CallIdentityInUse = 84,
    NoCallSuspended = 85,
    CallHavingTheRequestedCallIdentityHasBeenCleared = 86,
    UserNotMemberOfCUG = 87,
    IncompatibleDestination = 88,
    DestinationNumberMissingAndDCNotSubscribed = 90,
    InvalidTransitNetworkSelection = 91,
    InvalidMessageUnspecified = 95,
    MandatoryInformationElementIsMissing = 96,
    MessageTypeNonExistentOrNotImplemented = 97,
    MessageNotCompatibleWithCallStateOrMessageTypeNonExistentOrNotImplemented = 98,
    InformationElementNonExistentOrNotImplemented = 99,
    InvalidInformationElementContents = 100,
    MessageNotCompatibleWithCallState = 101,
    CallTerminatedWhenTimerExpiredARecoveryRoutineExecutedToRecoverFromTheError = 102,
    ParameterNonExistentOrNotImplemented = 103,
    MessageWithUnrecognizedParameterDiscarded = 110,
    ProtocolErrorUnspecified = 111,
    PrecedenceLevelExceeded = 122,
    DeviceNotPreemptable = 123,
}

#[derive(FromPrimitive)]
enum RedirectReasonCode {
    Unknown = 0,
    CallForwardBusy = 1,
    CallForwardNoReply = 2,
    CallTransfer = 4,
    CallPickup = 5,
    CallPark = 7,
    CallParkPickup = 8,
    CPEOutOfOrder = 9,
    CallForward = 10,
    CallParkReversion = 11,
    CallForwardAll = 15,
    CallDeflection = 18,
}
