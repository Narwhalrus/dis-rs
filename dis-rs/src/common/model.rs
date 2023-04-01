use crate::common::entity_state::model::EntityState;
use crate::common::{BodyInfo, Interaction};
use crate::common::acknowledge::model::Acknowledge;
use crate::common::action_request::model::ActionRequest;
use crate::common::action_response::model::ActionResponse;
use crate::common::attribute::model::Attribute;
use crate::common::collision::model::Collision;
use crate::common::collision_elastic::model::CollisionElastic;
use crate::common::create_entity::model::CreateEntity;
use crate::common::data::model::Data;
use crate::common::data_query::model::DataQuery;
use crate::common::designator::model::Designator;
use crate::common::detonation::model::Detonation;
use crate::common::electromagnetic_emission::model::ElectromagneticEmission;
use crate::common::entity_state_update::model::EntityStateUpdate;
use crate::common::event_report::model::EventReport;
use crate::common::other::model::Other;
use crate::common::fire::model::Fire;
use crate::common::receiver::model::Receiver;
use crate::common::remove_entity::model::RemoveEntity;
use crate::common::set_data::model::SetData;
use crate::common::signal::model::Signal;
use crate::common::start_resume::model::StartResume;
use crate::common::stop_freeze::model::StopFreeze;
use crate::common::transmitter::model::Transmitter;
use crate::enumerations::{Country, EntityKind, ExplosiveMaterialCategories, MunitionDescriptorFuse, MunitionDescriptorWarhead, PduType, PlatformDomain, ProtocolFamily, ProtocolVersion, VariableRecordType};
use crate::v7::model::PduStatus;
use crate::constants::{NO_REMAINDER, PDU_HEADER_LEN_BYTES};
use crate::fixed_parameters::{NO_APPLIC, NO_ENTITY, NO_SITE};

pub struct Pdu {
    pub header : PduHeader,
    pub body : PduBody,
}

impl Pdu {
    pub fn finalize_from_parts(header: PduHeader, body: PduBody, time_stamp: u32) -> Self {
        Self {
            header: header
                .with_time_stamp(time_stamp)
                .with_length(body.body_length() as u16),
            body,
        }
    }
}

impl Interaction for Pdu {
    fn originator(&self) -> Option<&EntityId> {
        self.body.originator()
    }

    fn receiver(&self) -> Option<&EntityId> {
        self.body.receiver()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PduHeader {
    pub protocol_version : ProtocolVersion,
    pub exercise_id : u8,
    pub pdu_type : PduType,
    pub protocol_family : ProtocolFamily,
    pub time_stamp : u32,
    pub pdu_length : u16,
    pub pdu_status : Option<PduStatus>,
    pub padding : u16,
}

impl PduHeader {
    pub fn new(protocol_version: ProtocolVersion, exercise_id: u8, pdu_type: PduType, protocol_family: ProtocolFamily) -> Self {
        Self {
            protocol_version,
            exercise_id,
            pdu_type,
            protocol_family,
            time_stamp: 0u32,
            pdu_length: 0u16,
            pdu_status: None,
            padding: 0u16,
        }
    }

    pub fn new_v6(exercise_id: u8, pdu_type: PduType) -> Self {
        PduHeader::new(ProtocolVersion::IEEE1278_1A1998, exercise_id, pdu_type, pdu_type.into())
    }

    pub fn new_v7(exercise_id: u8, pdu_type: PduType) -> Self {
        PduHeader::new(ProtocolVersion::IEEE1278_12012, exercise_id, pdu_type, pdu_type.into())
    }

    pub fn with_time_stamp(mut self, time_stamp: u32) -> Self {
        self.time_stamp = time_stamp;
        self
    }

    pub fn with_length(mut self, body_length: u16) -> Self {
        self.pdu_length = PDU_HEADER_LEN_BYTES + body_length;
        self
    }

    pub fn with_pdu_status(mut self, pdu_status: PduStatus) -> Self {
        self.pdu_status = Some(pdu_status);
        self
    }
}

pub enum PduBody {
    Other(Other),
    EntityState(EntityState),
    Fire(Fire),
    Detonation(Detonation),
    Collision(Collision),
    ServiceRequest,
    ResupplyOffer,
    ResupplyReceived,
    ResupplyCancel,
    RepairComplete,
    RepairResponse,
    CreateEntity(CreateEntity),
    RemoveEntity(RemoveEntity),
    StartResume(StartResume),
    StopFreeze(StopFreeze),
    Acknowledge(Acknowledge),
    ActionRequest(ActionRequest),
    ActionResponse(ActionResponse),
    DataQuery(DataQuery),
    SetData(SetData),
    Data(Data),
    EventReport(EventReport),
    Comment,
    ElectromagneticEmission(ElectromagneticEmission),
    Designator(Designator),
    Transmitter(Transmitter),
    Signal(Signal),
    Receiver(Receiver),
    IFF,
    UnderwaterAcoustic,
    SupplementalEmissionEntityState,
    IntercomSignal,
    IntercomControl,
    AggregateState,
    IsGroupOf,
    TransferOwnership,
    IsPartOf,
    MinefieldState,
    MinefieldQuery,
    MinefieldData,
    MinefieldResponseNACK,
    EnvironmentalProcess,
    GriddedData,
    PointObjectState,
    LinearObjectState,
    ArealObjectState,
    TSPI,
    Appearance,
    ArticulatedParts,
    LEFire,
    LEDetonation,
    CreateEntityR,
    RemoveEntityR,
    StartResumeR,
    StopFreezeR,
    AcknowledgeR,
    ActionRequestR,
    ActionResponseR,
    DataQueryR,
    SetDataR,
    DataR,
    EventReportR,
    CommentR,
    RecordR,
    SetRecordR,
    RecordQueryR,
    CollisionElastic(CollisionElastic),
    EntityStateUpdate(EntityStateUpdate),
    DirectedEnergyFire,
    EntityDamageStatus,
    InformationOperationsAction,
    InformationOperationsReport,
    Attribute(Attribute),
}

impl BodyInfo for PduBody {
    fn body_length(&self) -> u16 {
        match self {
            PduBody::Other(body) => { body.body_length() }
            PduBody::EntityState(body) => { body.body_length() }
            PduBody::Fire(body) => { body.body_length() }
            PduBody::Detonation(body) => { body.body_length() }
            PduBody::Collision(body) => { body.body_length() }
            PduBody::ServiceRequest => { 0 }
            PduBody::ResupplyOffer => { 0 }
            PduBody::ResupplyReceived => { 0 }
            PduBody::ResupplyCancel => { 0 }
            PduBody::RepairComplete => { 0 }
            PduBody::RepairResponse => { 0 }
            PduBody::CreateEntity(body) => { body.body_length() }
            PduBody::RemoveEntity(body) => { body.body_length() }
            PduBody::StartResume(body) => { body.body_length() }
            PduBody::StopFreeze(body) => { body.body_length() }
            PduBody::Acknowledge(body) => { body.body_length() }
            PduBody::ActionRequest(body) => { body.body_length() }
            PduBody::ActionResponse(body) => { body.body_length() }
            PduBody::DataQuery(body) => { body.body_length() }
            PduBody::SetData(body) => { body.body_length() }
            PduBody::Data(body) => { body.body_length() }
            PduBody::EventReport(body) => { body.body_length() }
            PduBody::Comment => { 0 }
            PduBody::ElectromagneticEmission(body) => { body.body_length() }
            PduBody::Designator(body) => { body.body_length() }
            PduBody::Transmitter(body) => { body.body_length() }
            PduBody::Signal(body) => { body.body_length() }
            PduBody::Receiver(body) => { body.body_length() }
            PduBody::IFF => { 0 }
            PduBody::UnderwaterAcoustic => { 0 }
            PduBody::SupplementalEmissionEntityState => { 0 }
            PduBody::IntercomSignal => { 0 }
            PduBody::IntercomControl => { 0 }
            PduBody::AggregateState => { 0 }
            PduBody::IsGroupOf => { 0 }
            PduBody::TransferOwnership => { 0 }
            PduBody::IsPartOf => { 0 }
            PduBody::MinefieldState => { 0 }
            PduBody::MinefieldQuery => { 0 }
            PduBody::MinefieldData => { 0 }
            PduBody::MinefieldResponseNACK => { 0 }
            PduBody::EnvironmentalProcess => { 0 }
            PduBody::GriddedData => { 0 }
            PduBody::PointObjectState => { 0 }
            PduBody::LinearObjectState => { 0 }
            PduBody::ArealObjectState => { 0 }
            PduBody::TSPI => { 0 }
            PduBody::Appearance => { 0 }
            PduBody::ArticulatedParts => { 0 }
            PduBody::LEFire => { 0 }
            PduBody::LEDetonation => { 0 }
            PduBody::CreateEntityR => { 0 }
            PduBody::RemoveEntityR => { 0 }
            PduBody::StartResumeR => { 0 }
            PduBody::StopFreezeR => { 0 }
            PduBody::AcknowledgeR => { 0 }
            PduBody::ActionRequestR => { 0 }
            PduBody::ActionResponseR => { 0 }
            PduBody::DataQueryR => { 0 }
            PduBody::SetDataR => { 0 }
            PduBody::DataR => { 0 }
            PduBody::EventReportR => { 0 }
            PduBody::CommentR => { 0 }
            PduBody::RecordR => { 0 }
            PduBody::SetRecordR => { 0 }
            PduBody::RecordQueryR => { 0 }
            PduBody::CollisionElastic(body) => { body.body_length() }
            PduBody::EntityStateUpdate(body) => { body.body_length() }
            PduBody::DirectedEnergyFire => { 0 }
            PduBody::EntityDamageStatus => { 0 }
            PduBody::InformationOperationsAction => { 0 }
            PduBody::InformationOperationsReport => { 0 }
            PduBody::Attribute(body) => { body.body_length() }
        }
    }

    fn body_type(&self) -> PduType {
        match self {
            PduBody::Other(body) => { body.body_type() }
            PduBody::EntityState(body) => { body.body_type() }
            PduBody::Fire(body) => { body.body_type() }
            PduBody::Detonation(body) => { body.body_type() }
            PduBody::Collision(body) => { body.body_type() }
            PduBody::ServiceRequest => { PduType::ServiceRequest }
            PduBody::ResupplyOffer => { PduType::ResupplyOffer }
            PduBody::ResupplyReceived => { PduType::ResupplyReceived }
            PduBody::ResupplyCancel => { PduType::ResupplyCancel }
            PduBody::RepairComplete => { PduType::RepairComplete }
            PduBody::RepairResponse => { PduType::RepairResponse }
            PduBody::CreateEntity(body) => { body.body_type() }
            PduBody::RemoveEntity(body) => { body.body_type() }
            PduBody::StartResume(body) => { body.body_type() }
            PduBody::StopFreeze(body) => { body.body_type() }
            PduBody::Acknowledge(body) => { body.body_type() }
            PduBody::ActionRequest(body) => { body.body_type() }
            PduBody::ActionResponse(body) => { body.body_type() }
            PduBody::DataQuery(body) => { body.body_type() }
            PduBody::SetData(body) => { body.body_type() }
            PduBody::Data(body) => { body.body_type() }
            PduBody::EventReport(body) => { body.body_type() }
            PduBody::Comment => { PduType::Comment }
            PduBody::ElectromagneticEmission(body) => { body.body_type() }
            PduBody::Designator(body) => { body.body_type() }
            PduBody::Transmitter(body) => { body.body_type() }
            PduBody::Signal(body) => { body.body_type() }
            PduBody::Receiver(body) => { body.body_type() }
            PduBody::IFF => { PduType::IFF }
            PduBody::UnderwaterAcoustic => { PduType::UnderwaterAcoustic }
            PduBody::SupplementalEmissionEntityState => { PduType::SupplementalEmissionEntityState }
            PduBody::IntercomSignal => { PduType::IntercomSignal }
            PduBody::IntercomControl => { PduType::IntercomControl }
            PduBody::AggregateState => { PduType::AggregateState }
            PduBody::IsGroupOf => { PduType::IsGroupOf }
            PduBody::TransferOwnership => { PduType::TransferOwnership }
            PduBody::IsPartOf => { PduType::IsPartOf }
            PduBody::MinefieldState => { PduType::MinefieldState }
            PduBody::MinefieldQuery => { PduType::MinefieldQuery }
            PduBody::MinefieldData => { PduType::MinefieldData }
            PduBody::MinefieldResponseNACK => { PduType::MinefieldResponseNACK }
            PduBody::EnvironmentalProcess => { PduType::EnvironmentalProcess }
            PduBody::GriddedData => { PduType::GriddedData }
            PduBody::PointObjectState => { PduType::PointObjectState }
            PduBody::LinearObjectState => { PduType::LinearObjectState }
            PduBody::ArealObjectState => { PduType::ArealObjectState }
            PduBody::TSPI => { PduType::TSPI }
            PduBody::Appearance => { PduType::Appearance }
            PduBody::ArticulatedParts => { PduType::ArticulatedParts }
            PduBody::LEFire => { PduType::LEFire }
            PduBody::LEDetonation => { PduType::LEDetonation }
            PduBody::CreateEntityR => { PduType::CreateEntityR }
            PduBody::RemoveEntityR => { PduType::RemoveEntityR }
            PduBody::StartResumeR => { PduType::StartResumeR }
            PduBody::StopFreezeR => { PduType::StopFreezeR }
            PduBody::AcknowledgeR => { PduType::AcknowledgeR }
            PduBody::ActionRequestR => { PduType::ActionRequestR }
            PduBody::ActionResponseR => { PduType::ActionResponseR }
            PduBody::DataQueryR => { PduType::DataQueryR }
            PduBody::SetDataR => { PduType::SetDataR }
            PduBody::DataR => { PduType::DataR }
            PduBody::EventReportR => { PduType::EventReportR }
            PduBody::CommentR => { PduType::CommentR }
            PduBody::RecordR => { PduType::RecordR }
            PduBody::SetRecordR => { PduType::SetRecordR }
            PduBody::RecordQueryR => { PduType::RecordQueryR }
            PduBody::CollisionElastic(body) => { body.body_type() }
            PduBody::EntityStateUpdate(body) => { body.body_type() }
            PduBody::DirectedEnergyFire => { PduType::DirectedEnergyFire }
            PduBody::EntityDamageStatus => { PduType::EntityDamageStatus }
            PduBody::InformationOperationsAction => { PduType::InformationOperationsAction }
            PduBody::InformationOperationsReport => { PduType::InformationOperationsReport }
            PduBody::Attribute(body) => { body.body_type() }
        }
    }
}

impl Interaction for PduBody {
    fn originator(&self) -> Option<&EntityId> {
        match self {
            PduBody::Other(body) => { body.originator() }
            PduBody::EntityState(body) => { body.originator() }
            PduBody::Fire(body) => { body.originator() }
            PduBody::Detonation(body) => { body.originator() }
            PduBody::Collision(body) => { body.originator() }
            PduBody::ServiceRequest => { None }
            PduBody::ResupplyOffer => { None }
            PduBody::ResupplyReceived => { None }
            PduBody::ResupplyCancel => { None }
            PduBody::RepairComplete => { None }
            PduBody::RepairResponse => { None }
            PduBody::CreateEntity(body) => { body.originator() }
            PduBody::RemoveEntity(body) => { body.originator() }
            PduBody::StartResume(body) => { body.originator() }
            PduBody::StopFreeze(body) => { body.originator() }
            PduBody::Acknowledge(body) => { body.originator() }
            PduBody::ActionRequest(body) => { body.originator() }
            PduBody::ActionResponse(body) => { body.originator() }
            PduBody::DataQuery(body) => { body.originator() }
            PduBody::SetData(body) => { body.originator() }
            PduBody::Data(body) => { body.originator() }
            PduBody::EventReport(body) => { body.originator() }
            PduBody::Comment => { None }
            PduBody::ElectromagneticEmission(body) => { body.originator() }
            PduBody::Designator(body) => { body.originator() }
            PduBody::Transmitter(body) => { body.originator() }
            PduBody::Signal(body) => { body.originator() }
            PduBody::Receiver(body) => { body.originator() }
            PduBody::IFF => { None }
            PduBody::UnderwaterAcoustic => { None }
            PduBody::SupplementalEmissionEntityState => { None }
            PduBody::IntercomSignal => { None }
            PduBody::IntercomControl => { None }
            PduBody::AggregateState => { None }
            PduBody::IsGroupOf => { None }
            PduBody::TransferOwnership => { None }
            PduBody::IsPartOf => { None }
            PduBody::MinefieldState => { None }
            PduBody::MinefieldQuery => { None }
            PduBody::MinefieldData => { None }
            PduBody::MinefieldResponseNACK => { None }
            PduBody::EnvironmentalProcess => { None }
            PduBody::GriddedData => { None }
            PduBody::PointObjectState => { None }
            PduBody::LinearObjectState => { None }
            PduBody::ArealObjectState => { None }
            PduBody::TSPI => { None }
            PduBody::Appearance => { None }
            PduBody::ArticulatedParts => { None }
            PduBody::LEFire => { None }
            PduBody::LEDetonation => { None }
            PduBody::CreateEntityR => { None }
            PduBody::RemoveEntityR => { None }
            PduBody::StartResumeR => { None }
            PduBody::StopFreezeR => { None }
            PduBody::AcknowledgeR => { None }
            PduBody::ActionRequestR => { None }
            PduBody::ActionResponseR => { None }
            PduBody::DataQueryR => { None }
            PduBody::SetDataR => { None }
            PduBody::DataR => { None }
            PduBody::EventReportR => { None }
            PduBody::CommentR => { None }
            PduBody::RecordR => { None }
            PduBody::SetRecordR => { None }
            PduBody::RecordQueryR => { None }
            PduBody::CollisionElastic(body) => { body.originator() }
            PduBody::EntityStateUpdate(body) => { body.originator() }
            PduBody::DirectedEnergyFire => { None }
            PduBody::EntityDamageStatus => { None }
            PduBody::InformationOperationsAction => { None }
            PduBody::InformationOperationsReport => { None }
            PduBody::Attribute(body) => { body.originator() }
        }
    }

    fn receiver(&self) -> Option<&EntityId> {
        match self {
            PduBody::Other(body) => { body.receiver() }
            PduBody::EntityState(body) => { body.receiver() }
            PduBody::Fire(body) => { body.receiver() }
            PduBody::Detonation(body) => { body.receiver() }
            PduBody::Collision(body) => { body.receiver() }
            PduBody::ServiceRequest => { None }
            PduBody::ResupplyOffer => { None }
            PduBody::ResupplyReceived => { None }
            PduBody::ResupplyCancel => { None }
            PduBody::RepairComplete => { None }
            PduBody::RepairResponse => { None }
            PduBody::CreateEntity(body) => { body.receiver() }
            PduBody::RemoveEntity(body) => { body.receiver() }
            PduBody::StartResume(body) => { body.receiver() }
            PduBody::StopFreeze(body) => { body.receiver() }
            PduBody::Acknowledge(body) => { body.receiver() }
            PduBody::ActionRequest(body) => { body.receiver() }
            PduBody::ActionResponse(body) => { body.receiver() }
            PduBody::DataQuery(body) => { body.receiver() }
            PduBody::SetData(body) => { body.receiver() }
            PduBody::Data(body) => { body.receiver() }
            PduBody::EventReport(body) => { body.receiver() }
            PduBody::Comment => { None }
            PduBody::ElectromagneticEmission(body) => { body.receiver() }
            PduBody::Designator(body) => { body.receiver() }
            PduBody::Transmitter(body) => { body.receiver() }
            PduBody::Signal(body) => { body.receiver() }
            PduBody::Receiver(body) => { body.receiver() }
            PduBody::IFF => { None }
            PduBody::UnderwaterAcoustic => { None }
            PduBody::SupplementalEmissionEntityState => { None }
            PduBody::IntercomSignal => { None }
            PduBody::IntercomControl => { None }
            PduBody::AggregateState => { None }
            PduBody::IsGroupOf => { None }
            PduBody::TransferOwnership => { None }
            PduBody::IsPartOf => { None }
            PduBody::MinefieldState => { None }
            PduBody::MinefieldQuery => { None }
            PduBody::MinefieldData => { None }
            PduBody::MinefieldResponseNACK => { None }
            PduBody::EnvironmentalProcess => { None }
            PduBody::GriddedData => { None }
            PduBody::PointObjectState => { None }
            PduBody::LinearObjectState => { None }
            PduBody::ArealObjectState => { None }
            PduBody::TSPI => { None }
            PduBody::Appearance => { None }
            PduBody::ArticulatedParts => { None }
            PduBody::LEFire => { None }
            PduBody::LEDetonation => { None }
            PduBody::CreateEntityR => { None }
            PduBody::RemoveEntityR => { None }
            PduBody::StartResumeR => { None }
            PduBody::StopFreezeR => { None }
            PduBody::AcknowledgeR => { None }
            PduBody::ActionRequestR => { None }
            PduBody::ActionResponseR => { None }
            PduBody::DataQueryR => { None }
            PduBody::SetDataR => { None }
            PduBody::DataR => { None }
            PduBody::EventReportR => { None }
            PduBody::CommentR => { None }
            PduBody::RecordR => { None }
            PduBody::SetRecordR => { None }
            PduBody::RecordQueryR => { None }
            PduBody::CollisionElastic(body) => { body.receiver() }
            PduBody::EntityStateUpdate(body) => { body.receiver() }
            PduBody::DirectedEnergyFire => { None }
            PduBody::EntityDamageStatus => { None }
            PduBody::InformationOperationsAction => { None }
            PduBody::InformationOperationsReport => { None }
            PduBody::Attribute(body) => { body.receiver() }
        }
    }
}

impl From<PduType> for ProtocolFamily {
    fn from(pdu_type: PduType) -> Self {
        match pdu_type {
            PduType::Other => ProtocolFamily::Other,
            PduType::EntityState => ProtocolFamily::EntityInformationInteraction,
            PduType::Fire => ProtocolFamily::Warfare,
            PduType::Detonation => ProtocolFamily::Warfare,
            PduType::Collision => ProtocolFamily::EntityInformationInteraction,
            PduType::ServiceRequest => ProtocolFamily::Logistics,
            PduType::ResupplyOffer => ProtocolFamily::Logistics,
            PduType::ResupplyReceived => ProtocolFamily::Logistics,
            PduType::ResupplyCancel => ProtocolFamily::Logistics,
            PduType::RepairComplete => ProtocolFamily::Logistics,
            PduType::RepairResponse => ProtocolFamily::Logistics,
            PduType::CreateEntity => ProtocolFamily::SimulationManagement,
            PduType::RemoveEntity => ProtocolFamily::SimulationManagement,
            PduType::StartResume => ProtocolFamily::SimulationManagement,
            PduType::StopFreeze => ProtocolFamily::SimulationManagement,
            PduType::Acknowledge => ProtocolFamily::SimulationManagement,
            PduType::ActionRequest => ProtocolFamily::SimulationManagement,
            PduType::ActionResponse => ProtocolFamily::SimulationManagement,
            PduType::DataQuery => ProtocolFamily::SimulationManagement,
            PduType::SetData => ProtocolFamily::SimulationManagement,
            PduType::Data => ProtocolFamily::SimulationManagement,
            PduType::EventReport => ProtocolFamily::SimulationManagement,
            PduType::Comment => ProtocolFamily::SimulationManagement,
            PduType::ElectromagneticEmission => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::Designator => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::Transmitter => ProtocolFamily::RadioCommunications,
            PduType::Signal => ProtocolFamily::RadioCommunications,
            PduType::Receiver => ProtocolFamily::RadioCommunications,
            PduType::IFF => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::UnderwaterAcoustic => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::SupplementalEmissionEntityState => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::IntercomSignal => ProtocolFamily::RadioCommunications,
            PduType::IntercomControl => ProtocolFamily::RadioCommunications,
            PduType::AggregateState => ProtocolFamily::EntityManagement,
            PduType::IsGroupOf => ProtocolFamily::EntityManagement,
            PduType::TransferOwnership => ProtocolFamily::EntityManagement,
            PduType::IsPartOf => ProtocolFamily::EntityManagement,
            PduType::MinefieldState => ProtocolFamily::Minefield,
            PduType::MinefieldQuery => ProtocolFamily::Minefield,
            PduType::MinefieldData => ProtocolFamily::Minefield,
            PduType::MinefieldResponseNACK => ProtocolFamily::Minefield,
            PduType::EnvironmentalProcess => ProtocolFamily::SyntheticEnvironment,
            PduType::GriddedData => ProtocolFamily::SyntheticEnvironment,
            PduType::PointObjectState => ProtocolFamily::SyntheticEnvironment,
            PduType::LinearObjectState => ProtocolFamily::SyntheticEnvironment,
            PduType::ArealObjectState => ProtocolFamily::SyntheticEnvironment,
            PduType::TSPI => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::Appearance => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::ArticulatedParts => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::LEFire => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::LEDetonation => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::CreateEntityR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::RemoveEntityR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::StartResumeR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::StopFreezeR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::AcknowledgeR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::ActionRequestR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::ActionResponseR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::DataQueryR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::SetDataR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::DataR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::EventReportR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::CommentR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::RecordR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::SetRecordR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::RecordQueryR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::CollisionElastic => ProtocolFamily::EntityInformationInteraction,
            PduType::EntityStateUpdate => ProtocolFamily::EntityInformationInteraction,
            PduType::DirectedEnergyFire => ProtocolFamily::Warfare,
            PduType::EntityDamageStatus => ProtocolFamily::Warfare,
            PduType::InformationOperationsAction => ProtocolFamily::InformationOperations,
            PduType::InformationOperationsReport => ProtocolFamily::InformationOperations,
            PduType::Attribute => ProtocolFamily::EntityInformationInteraction,
            PduType::Unspecified(unspecified_value) => ProtocolFamily::Unspecified(unspecified_value)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SimulationAddress {
    pub site_id : u16,
    pub application_id : u16,
}

impl SimulationAddress {
    pub fn new(site_id: u16, application_id: u16) -> Self {
        SimulationAddress {
            site_id,
            application_id
        }
    }
}

impl Default for SimulationAddress {
    fn default() -> Self {
        Self {
            site_id: NO_SITE,
            application_id: NO_APPLIC
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityId {
    pub simulation_address : SimulationAddress,
    pub entity_id : u16
}

impl Default for EntityId {
    fn default() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            entity_id: NO_ENTITY
        }
    }
}

impl EntityId {
    pub fn new(site_id : u16, application_id : u16, entity_id : u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_id,
                application_id
            },
            entity_id
        }
    }

    pub fn new_sim_address(simulation_address: SimulationAddress, entity_id : u16) -> Self {
        Self {
            simulation_address,
            entity_id
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EventId {
    pub simulation_address : SimulationAddress,
    pub event_id : u16
}

impl EventId {
    pub fn new(simulation_address: SimulationAddress, event_id: u16) -> Self {
        Self {
            simulation_address,
            event_id
        }
    }

    pub fn new_sim_address(simulation_address: SimulationAddress, event_id : u16) -> Self {
        Self {
            simulation_address,
            event_id
        }
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            event_id: NO_ENTITY
        }
    }
}

#[derive(Default)]
pub struct VectorF32 {
    pub first_vector_component : f32,
    pub second_vector_component : f32,
    pub third_vector_component : f32,
}

impl VectorF32 {
    pub fn new(first: f32, second: f32, third: f32) -> Self {
        VectorF32 {
            first_vector_component: first,
            second_vector_component: second,
            third_vector_component: third
        }
    }

    pub fn with_first(mut self, first: f32) -> Self {
        self.first_vector_component = first;
        self
    }

    pub fn with_second(mut self, second: f32) -> Self {
        self.first_vector_component = second;
        self
    }

    pub fn with_third(mut self, third: f32) -> Self {
        self.first_vector_component = third;
        self
    }
}

#[derive(Default)]
pub struct Location {
    pub x_coordinate : f64,
    pub y_coordinate : f64,
    pub z_coordinate : f64,
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Location {
            x_coordinate: x,
            y_coordinate: y,
            z_coordinate: z
        }
    }

    pub fn with_x(mut self, x: f64) -> Self {
        self.x_coordinate = x;
        self
    }

    pub fn with_y(mut self, y: f64) -> Self {
        self.y_coordinate = y;
        self
    }

    pub fn with_z(mut self, z: f64) -> Self {
        self.z_coordinate = z;
        self
    }
}

#[derive(Default)]
pub struct Orientation {
    pub psi : f32,
    pub theta : f32,
    pub phi : f32,
}

impl Orientation {
    pub fn new(psi: f32, theta: f32, phi: f32) -> Self {
        Orientation {
            psi,
            theta,
            phi
        }
    }

    pub fn with_psi(mut self, psi: f32) -> Self {
        self.psi = psi;
        self
    }

    pub fn with_theta(mut self, theta: f32) -> Self {
        self.theta = theta;
        self
    }

    pub fn with_phi(mut self, phi: f32) -> Self {
        self.phi = phi;
        self
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct EntityType {
    pub kind : EntityKind,
    pub domain : PlatformDomain,
    pub country : Country,
    pub category : u8,
    pub subcategory : u8,
    pub specific : u8,
    pub extra : u8,
}

impl EntityType {
    pub fn with_kind(mut self, kind: EntityKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_domain(mut self, domain: PlatformDomain) -> Self {
        self.domain = domain;
        self
    }

    pub fn with_country(mut self, country: Country) -> Self {
        self.country = country;
        self
    }

    pub fn with_category(mut self, category: u8) -> Self {
        self.category = category;
        self
    }

    pub fn with_subcategory(mut self, subcategory: u8) -> Self {
        self.subcategory = subcategory;
        self
    }

    pub fn with_specific(mut self, specific: u8) -> Self {
        self.specific = specific;
        self
    }

    pub fn with_extra(mut self, extra: u8) -> Self {
        self.extra = extra;
        self
    }
}

pub enum DescriptorRecord {
    Munition { entity_type: EntityType, munition: MunitionDescriptor },
    Expendable { entity_type: EntityType },
    Explosion { entity_type: EntityType, explosive_material: ExplosiveMaterialCategories, explosive_force: f32 }
}

impl DescriptorRecord {
    pub fn new_munition(entity_type: EntityType, munition: MunitionDescriptor) -> Self {
        DescriptorRecord::Munition {
            entity_type,
            munition
        }
    }

    pub fn new_expendable(entity_type: EntityType) -> Self {
        DescriptorRecord::Expendable {
            entity_type
        }
    }

    pub fn new_explosion(entity_type: EntityType, explosive_material: ExplosiveMaterialCategories, explosive_force: f32) -> Self {
        DescriptorRecord::Explosion {
            entity_type,
            explosive_material,
            explosive_force
        }
    }
}

impl Default for DescriptorRecord {
    fn default() -> Self {
        DescriptorRecord::new_munition(
            EntityType::default(),
            MunitionDescriptor::default())
    }
}

#[derive(Default)]
pub struct MunitionDescriptor {
    pub warhead : MunitionDescriptorWarhead,
    pub fuse : MunitionDescriptorFuse,
    pub quantity : u16,
    pub rate : u16,
}

impl MunitionDescriptor {
    pub fn with_warhead(mut self, warhead: MunitionDescriptorWarhead) -> Self {
        self.warhead = warhead;
        self
    }

    pub fn with_fuse(mut self, fuse: MunitionDescriptorFuse) -> Self {
        self.fuse = fuse;
        self
    }

    pub fn with_quantity(mut self, quantity: u16) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn with_rate(mut self, rate: u16) -> Self {
        self.rate = rate;
        self
    }
}

#[derive(Default)]
pub struct ClockTime {
    pub hour: i32,
    pub time_past_hour: u32,
}

impl ClockTime {
    pub fn new(hour: i32, time_past_hour: u32) -> Self {
        Self {
            hour,
            time_past_hour,
        }
    }
}

pub struct DatumSpecification {
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl DatumSpecification {
    pub fn new(fixed_datum_records: Vec<FixedDatum>, variable_datum_records: Vec<VariableDatum>) -> Self {
        Self {
            fixed_datum_records,
            variable_datum_records
        }
    }
}

pub const FIXED_DATUM_LENGTH: u16 = 8;
pub const BASE_VARIABLE_DATUM_LENGTH: u16 = 8;

pub struct FixedDatum {
    pub datum_id: VariableRecordType,
    pub datum_value: u32,
}

impl FixedDatum {
    pub fn new(datum_id: VariableRecordType, datum_value: u32) -> Self {
        Self {
            datum_id,
            datum_value
        }
    }
}

pub struct VariableDatum {
    pub datum_id: VariableRecordType,
    pub datum_value: Vec<u8>,
}

impl VariableDatum {
    pub fn new(datum_id: VariableRecordType, datum_value: Vec<u8>) -> Self {
        Self {
            datum_id,
            datum_value
        }
    }
}

/// Struct to hold the length in bytes of parts of a padded record.
/// `data_length_bytes` + `padding_length_bytes` = `record_length_bytes`.
pub struct PaddedRecordLengths {
    pub data_length_bytes: usize,
    pub padding_length_bytes: usize,
    pub record_length_bytes: usize,
}

impl PaddedRecordLengths {
    pub fn new(data_length_bytes: usize,
               padding_length_bytes: usize,
               record_length_bytes: usize) -> Self {
        Self {
            data_length_bytes,
            padding_length_bytes,
            record_length_bytes
        }
    }
}

/// Calculates the length of a data record when padded to `pad_to_num_bytes` octets,
/// given that the length of the data in the record is `data_length_bytes`.
/// The function returns a tuple consisting of the length of the data, the lenght of the padding, and the total (padded) length of the record.
///
/// For example, a piece of data of 12 bytes that needs to be aligned to 16 bytes will have a
/// data length of 12 bytes, a padding of 4 bytes and a final length of 12 + 4 bytes. The function will return 16 in this case.
pub fn length_padded_to_num_bytes(data_length_bytes: usize, pad_to_num_bytes: usize) -> PaddedRecordLengths {
    let data_remaining_bytes = data_length_bytes % pad_to_num_bytes;
    let padding_bytes = pad_to_num_bytes - data_remaining_bytes;
    let padded_data_bytes = data_length_bytes + padding_bytes;
    assert_eq!(padded_data_bytes % pad_to_num_bytes, NO_REMAINDER,
               "The length for the data record is not aligned to {} octets. Data length is {} octets.", pad_to_num_bytes, data_length_bytes);

    PaddedRecordLengths::new(data_length_bytes, padding_bytes, padded_data_bytes)
}