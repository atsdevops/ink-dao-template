#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::felidaeDAO::{FelidaeDao,FelidaeDaoRef};

#[openbrush::contract]
pub mod felidaeDAO {
    // use sp_arithmetic::{FixedU128, SignedRounding, FixedI128}; 
    use fixed::{types::extra::{U3,U4}, FixedI128};
    use ink_prelude::string::{String, ToString};
    use ink_prelude::{vec,vec::Vec};
    use ink_storage::traits::StorageLayout;
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use dao_governance_token::dao_governance_token::{DaoGovernanceToken, DaoGovernanceTokenRef};
    use openbrush::contracts::ownable::OwnableError;
    use openbrush::{storage::Mapping, traits::{Storage}};
    use openbrush::{contracts::ownable::*};
    type ProjectId = u16;
    type TaskId = u16;
    type MemberId = u16;
    type Fix = FixedI128<U4>;
    pub type ResultTransaction<T> = core::result::Result<T, Error>;
    pub type ResultOwner<T> = core::result::Result<T, OwnableError>;
    pub type Result<T> = core::result::Result<T, Error>;

    const min_deposit_balance: Balance = 1_000_000_000;
    const SECONDS_PER_DAY: u64 = 86400;

    // const total_stake: u8 = 100;
    // const stake_t: u64  = 1 * 10u64.pow(12); // 1 token
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct FelidaeDao {
        ///dao authority
        dao_admin:AccountId,
        ///daoID
        dao_id:u8,
        ///dao info 
        dao_info:DaoInfo,
        /// token address => token info
        token_list_for_address: Mapping<AccountId, TokenInfo>,
        ///track the id's of member
        next_member_id: u16,
        ///AccountId => MemberInfo
        dao_member_list:Mapping<AccountId, MemberInfo>,
        /// ( DAO address , member_id ) => MemberInfo
        member_infoes_from_id: Mapping< MemberId, MemberInfo>,
        ///projectId => ProjectInfo
        dao_project_list:Mapping<ProjectId, ProjectInfo>,
        ///daoAddress => ProjectId
        next_project_id: u16,
        /// TaskId => TaskInfo
        dao_task_list:Mapping<TaskId, TaskInfo>,
        ///(daoAddress,ProjectId) => taskId
        next_task_id: u16,
        ///member address => stake
        staking_data: Mapping<AccountId, Stake>,
        ///interest rate
        total_stake: u8, // interest rate per cent
        ///stake duration
        staking_duration: Timestamp,
        ///nextdaoID
        next_dao_id:u16,
        ///dao_Address => DaoInfo
        dao_list:Mapping<AccountId,DaoInfo>

    }
       
    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct DaoInfo {
        dao_name: String,
        description: String,
        website:Option<String>,
        profile:Option<String>
    }
    #[derive(
        Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, PartialEq,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct MemberInfo {
        name: String,
        member_id: MemberId,
        member_status:MemberStatus,
        member_efficiency:u128,
        member_role:MemberRole,
        start_time:String,
        end_time:Option<String>,
        task_list:Vec<TaskId>,
        project_list:Vec<ProjectId>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, PartialEq)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct TokenInfo {
        token_type: TokenType,
        token_address: AccountId,
    }

    #[derive(
        Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum TokenType {
        GovernanceToken,
        Psp22,
        Psp34,
    }
    #[derive(
        Default,Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum MemberStatus {
        Active,
        Inactive,
        Terminated,
        #[default]
        None

    }

    #[derive(
        Default,Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum MemberRole {
        Creator,
        Recruiter,
        Supporter,
        Auditor,
        Marketer,
        Seller,
        Advisor,
        #[default]
        None

    }

    #[derive(
        Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, PartialEq,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ProjectInfo {
        project_id:ProjectId,
        name:String,
        creator:AccountId,
        project_status:ProjectStatus,
        assigned_to:AccountId,
        start_time:String,
        end_time:Option<String>,
        task_list:Vec<TaskId>,
        sprint:Sprint
    }
    #[derive(
        Default,Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum ProjectStatus {
        Active,
        Inactive,
        Completed,
        Incompelte,
        #[default]
        None
    }
    
    #[derive(
        Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, PartialEq,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct TaskInfo {
        task_id:TaskId,
        project_id:u16,
        name:String,
        creator:AccountId,
        task_status:TaskStatus,
        assigned_to:AccountId,
        task_type:TaskType,
        start_time:String,
        end_time:Option<String>,
        review:Option<ReviewStatus>,
        total_time_logged_in:Option<u16>,
    }
      
    #[derive(
        Default,Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum ReviewOpinion {
        /// Agree.
        YES,
        /// Reject.
        NO,
        #[default]
        None
    }

    #[derive(
        Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, PartialEq,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ReviewRecord {
        pub who: AccountId,
        pub meta: Vec<u8>,
        pub option: ReviewOpinion,
    }

    #[derive(
        Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, PartialEq,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ReviewStatus {
        pub records: ReviewRecord,
        
    }

    #[derive(
        scale::Encode, scale::Decode, Eq, PartialEq, Debug, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Stake {
        staked_amount: Balance,
        deposit_time: String,
        release_time: Option<String>,
    }


    #[derive(Clone, Debug, PartialEq, scale::Decode, scale::Encode)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
	pub struct Metadata {
		owner: AccountId,
        staking_duration:Timestamp,
        interest: u8,
	}
   
    #[derive(
        Default,scale::Encode, scale::Decode, Eq, PartialEq, Debug, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Sprint {
        project_id: ProjectId,
        start_date:String, 
        end_date:String, 
        action:u8,    
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// The Token Does Not Exists.
        TheTokenDoesNotExist,
        /// Invalid Operation.
        InvalidOperation,
        ///Not a Member
        NotAMember,
        /// Distribution is failure.
        DistributionIsFailure,
        /// Changing Token Status Is Failure.
        ChangingTokenStatusIsFailure,
        /// Withdrawing is Failure.
        WithdrawingIsFailure,
        /// Wrong Csv Data
        WrongCsvData,
        /// Tranfering Contract Balance is Failure
        TransferingContractBalanceIsFailure,
        /// Tranfering Contract Balance is Failure
        ThisFunctionCanBeCalledFromDaoManager,
         /// Not first member
         NotFirstMember,
         /// Target member does not exist.
         MemberDoesNotExist,
         /// Target member already exists.
         MemberAlreadyExists,
         /// Electoral Commissioner Data is mismatched.
         ElectoralCommissionerDataMismatch,
         /// Only Member does.
         OnlyMemberDoes,
         /// Only Electoral Commissioner
         OnlyElectoralCommissioner,
         /// Only Proposal Manager Address call this function.
         OnlyFromProposalManagerAddress,
         /// Csv Convert Failure
         CsvConvertFailure,
         /// Invalid Electoral Commissioner Count
         InvalidElectoralCommissionerCount,
         /// Invalid Delete Member Count
         InvalidDeleteMemberCount,
         /// At least one election commissioner
         AtLeastOneElectionCommissioner,
         /// Possible bug
         PossibleBug,
         ///Not a creator
         NotACreator,
         ///project does not exist
         ProjectDoesNotExist,
         ///Task does not exist
         TaskDoesNotExist,
         ///Not a Task Authority
         NotaTaskAuthority,
         ///Account does not exists
         AccountNotExist,
         ///Not Allowed
         NotAllowed,
         /// Only the owner can calim the refund
         InvalidRefundRequest, 
         /// Prevents multiple stake for same account, one person  one stake         
         AccountAlreadyExists, 
         ///deposits not sufficient
         DepositNotSufficient,
         ///self staking not allowed
         SelfStakingNotAllowed,
         ///can not reddem before the peroid 
         RedeemDurationNotReached,
         ///indufficient funds
         InsufficientContractBalance,
         ///transfer failed 
         TransferFailed,
         ///Not a Admin
         NotAAdmin,
         ///total ownership can not be 
         InvalidOwnershipPercentage,
         //add stake failed
         AddStakeFailed,
         ///Already staked
         AlreadyStaked,
         ///NotStaked
         NotStaked
    }   

    #[derive(
        Default,Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum TaskStatus {
        ToDO,
        InProgress,
        ReadyToPR,
        BackToCw, 
        DevVerfied,
        LiveDeployed,
        Closed,
        Completed,
        #[default]
        None
    }
    #[derive(
        Default,Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum TaskType {
        Bug,
        Feature,
        #[default]
        None
    }
    #[ink(event)]
    pub struct MemberAdded {
        dao_address:AccountId,
        member: AccountId,
        member_id: u16,
    }
    #[ink(event)]
    pub struct GovernanceTokenAdded {
        dao_address:AccountId,
        token_address: AccountId,
    }
    #[ink(event)]
    pub struct Transferred {
        from:Option<AccountId>,
        to: AccountId,
        value:Balance
    }
    #[ink(event)]
    pub struct MemberTerminated {
        dao_address:AccountId,
        member_address: AccountId,
        start_time:String,
        end_time:Option<String>,
    }

    #[ink(event)]
    pub struct MemberRoleUpdated {
        dao_address:AccountId,
        member_address: AccountId,
        new_role: MemberRole,
    }
    #[ink(event)]
    pub struct ProjectCreated {
        dao_address:AccountId,
        creator: AccountId,
        project_id: ProjectId,
        assigned_to:AccountId,
        start_time:String
    }
    #[ink(event)]
    pub struct ProjectStatusUpdated {
        project_id:ProjectId,
        status: ProjectStatus,
    }
    #[ink(event)]
    pub struct ProjectCompleted {
        project_id:ProjectId,
        start_time: String,
        end_time:Option<String>
    }

    #[ink(event)]
    pub struct TaskCreated {
        task_id:TaskId,
        project_id:ProjectId,
        creator:AccountId,
        assigned_to:AccountId,
        task_type:TaskType,
        start_time: String,
    }
   // Events to be propagated in response to some activities
   #[ink(event)]
   pub struct RedeemSuccessful {
       staker: AccountId,
       stake: Stake,
      
   }

   #[ink(event)]
   pub struct WithdrawSuccessful {
       staker: AccountId,
       stake: Stake,
    
   }

   #[ink(event)]
   pub struct DepositSuccessful {
       staker: AccountId,
       stake: Stake,
   }
      
   #[ink(event)]
   pub struct SprintAdded {
    project_id: ProjectId,
    start_date: String,
    end_date: String,
   }
 
    impl FelidaeDao {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(admin:AccountId, dao_name:String, website:Option<String>, profile:Option<String>,description:String) -> Self {
            let timestamp = ink_env::block_timestamp::<ink_env::DefaultEnvironment>();

            // let data = Stake {
            //     staked_amount: 0,
            //     deposit_time: timestamp,
            //     release_time: timestamp ,
            //     ownership_percentage:100,
            // };

           let mut feliada_dao = Self{
                dao_admin: admin,
                dao_id:0,
                dao_info: DaoInfo {
                    dao_name: dao_name,
                    description: description,
                    website:website,
                    profile:profile
                },
                dao_member_list: Mapping::default(),
                dao_project_list: Mapping::default(),
                token_list_for_address: Mapping::default(),
                next_project_id:0,
                member_infoes_from_id: Mapping::default(),
                dao_task_list: Mapping::default(),
                next_member_id:0,
                next_task_id:0,
                staking_data:Mapping::default(),
                total_stake: u8::default(),
                staking_duration: Timestamp::default(),
                dao_list:Mapping::default(),
                next_dao_id:0

            };
            // feliada_dao.staking_data.insert(&feliada_dao.dao_admin, &data);
            feliada_dao
        }

        #[ink(message)]
        pub fn get_dao_info(&self) -> DaoInfo {
            self.dao_info.clone()
        }

        #[ink(message)]
        pub fn add_dao_token(
            &mut self,
            dao_address: AccountId,
            token_type: TokenType,
            token_address: AccountId,
        ) -> Result<()> {
            if !self._is_calling_from_dao_manager() {
                return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
            }

            let token_info = TokenInfo {
                token_type: token_type,
                token_address: token_address,
            };
            // self.token_list_for_id.insert(
            //     &self.next_token_id,
            //     &token_info.clone()
            // );
            self.token_list_for_address.insert(&token_address, &token_info.clone());
            // self.next_token_id = self.next_token_id + 1;
            Self::env().emit_event(GovernanceTokenAdded {
                dao_address: dao_address,
                token_address: token_address,
            });
            Ok(())
        }

        /// Constructor that initializes the `bool` value to `false`.
        //
        // /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }
        #[ink(message)]
        pub fn get_token_list(&self,token_address: AccountId) -> Vec<TokenInfo> {
            let mut result: Vec<TokenInfo> = Vec::new();
            
                match self.token_list_for_address.get(&token_address) {
                    Some(value) => result.push(value),
                    None => (),
                }
            
            result
        }
     
        #[ink(message)]
        pub fn distribute_governance_token(&mut self, token_address: AccountId, to_address:AccountId, amount:Balance) -> Result<()> {
            if !self._is_calling_from_dao_manager() {
                return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
            }

            let token_info: TokenInfo = match self.token_list_for_address.get(&token_address) {
                Some(value) => value,
                None => return Err(Error::TheTokenDoesNotExist),
            };
                
            let mut instance: DaoGovernanceTokenRef = ink_env::call::FromAccountId::from_account_id(token_address);
            
            match instance.distribute_token(to_address ,amount) {
                Ok(()) =>{ 
                    
                    Self::env().emit_event(Transferred {
                        from: None,
                        to: to_address,
                        value: amount,
                    });

                    return Ok(())
                
                },
                Err(_e) => return Err(Error::DistributionIsFailure),
            }

        }
         

        // /// A message that can be called on instantiated contracts.
        // /// This one flips the value of the stored `bool` from `true`
        // /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
        }
        #[ink(message)]
        pub fn get_admin(&mut self)->AccountId {
           self.dao_admin.clone()
        }
        #[ink(message)]
        pub fn is_admin(&mut self,admin:AccountId)->bool {
          if self.dao_admin ==admin{
            return true
          }else{
            return false
          }

        }
        #[inline]
        fn _is_calling_from_dao_manager(&self) -> bool {
            self.env().caller() == self.dao_admin
        }
        #[ink(message)]
        pub fn get_contract_balance(&self) -> Balance {
            self.env().balance()
        }

            /// add  member.
            #[ink(message)]
            pub fn add_member(
                &mut self,
                dao_address: AccountId,
                member_address: AccountId,
                name: String,
            ) -> ResultTransaction<()> {
                let caller = self.env().caller();

                if caller != self.dao_admin {
                    return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
                }
                
                if self
                .dao_member_list
                .get(&(member_address))
                != None
            {
                ink_env::debug_println!("########## MEMBER EXISTS ALREADY Error.");
                return Err(Error::MemberAlreadyExists);
            }
         
                self.inline_add_member(dao_address, name, member_address);
                
                Ok(())
            }

            /// delete the member
            #[ink(message)]
            pub fn delete_member(&mut self, _dao_address: AccountId, member_address: AccountId, ) -> ResultTransaction<()> {
                if !self._is_calling_from_dao_manager() {
                    return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
                }
                self.inline_delete_member(_dao_address, member_address)
            }

            /// inline delete the member.
            #[inline]
            fn inline_delete_member(
            &mut self,
            dao_address: AccountId,
            member_address: AccountId,
            ) -> ResultTransaction<()> {
            let member_info = match self.dao_member_list.get(&(member_address)) {
            Some(value) => value,
            None => {
                ink_env::debug_println!("MemberDoesNotExist Error.");
                return Err(Error::MemberDoesNotExist);
            },
            };
            let next_member_id = member_info.member_id;
            self.member_infoes_from_id
            .remove(&next_member_id);
             self.dao_member_list.remove(&(member_address));
        
                Ok(())
            }

            /// terminate the member
            #[ink(message)]
            pub fn terminate_member(&mut self, _dao_address: AccountId, member_address: AccountId, ) -> ResultTransaction<()> {
                if !self._is_calling_from_dao_manager() {
                    return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
                }
                self.inline_terminate_member(_dao_address, member_address)
            }

            /// inline terminate the member.
            #[inline]
            fn inline_terminate_member(
            &mut self,
            dao_address: AccountId,
            member_address: AccountId,
            ) -> ResultTransaction<()> {
            let mut member_info = match self.dao_member_list.get(&(member_address)) {
            Some(value) => value,
            None => {
                ink_env::debug_println!("MemberDoesNotExist Error.");
                return Err(Error::MemberDoesNotExist);
            },
            };
            let next_member_id = member_info.member_id;

            let mut member_info_ = match self.member_infoes_from_id.get(&next_member_id) {
                Some(value) => value,
                None => {
                    ink_env::debug_println!("MemberDoesNotExist Error.");
                    return Err(Error::MemberDoesNotExist);
                },
                };
                
            self.dao_member_list
                .remove(&member_address);
            self.member_infoes_from_id
            .remove(&next_member_id);
            let time_now =  Self::env().block_timestamp();
            member_info.member_status = MemberStatus::Terminated;
            member_info.end_time = Some(time_now.to_string());


               self.dao_member_list
                .insert(&member_address, &member_info.clone());
               self.member_infoes_from_id
            .insert(&next_member_id, &member_info.clone());

            Self::env().emit_event(MemberTerminated {
                dao_address: dao_address,
                member_address: member_address,
                start_time: member_info.start_time,
                end_time: member_info.end_time,

            });
                Ok(())
            }


        #[inline]
        fn _convert_string_to_accountid(&self, account_str: &str) -> AccountId {
            let mut output = vec![0xFF; 35];
            bs58::decode(account_str).into(&mut output).unwrap();
            let cut_address_vec: Vec<_> = output.drain(1..33).collect();
            let mut array = [0; 32];
            let bytes = &cut_address_vec[..array.len()];
            array.copy_from_slice(bytes);
            let account_id: AccountId = array.into();
            account_id
        }

        #[inline]
        fn _convert_timestamp_to_date(&self, timestamp:Timestamp)  {
          let res =  Timestamp::from(timestamp);
        }
        

        #[inline]
        fn inline_add_member(
            &mut self,
            dao_address: AccountId,
            name: String,
            member_address: AccountId,
        ) {
            
            //calculate the start time 
            let mut task_list:Vec<TaskId> =Vec::new(); 
            let mut project_list:Vec<ProjectId> =Vec::new(); 
            let time_now =  Self::env().block_timestamp();
            let member_info = MemberInfo {
                name: name,
                member_id: self.next_member_id,
                member_status:MemberStatus::Active,
                member_efficiency:0,
                member_role:MemberRole::None,
                start_time:time_now.to_string(),
                end_time:None,
                task_list,
                project_list
            };

            self.member_infoes_from_id
            .insert(&self.next_member_id, &member_info.clone());
        
            self.dao_member_list
                .insert(&member_address, &member_info.clone());
            self.next_member_id = self.next_member_id + 1;

            Self::env().emit_event(MemberAdded {
                dao_address: dao_address,
                member: member_address,
                member_id: self.next_member_id,
            });
        }

        /// get member info 
        #[ink(message)]
        pub fn get_member_info(&self,member_address:AccountId) -> ResultTransaction<MemberInfo>  {
            
            match self.dao_member_list.get(&(member_address)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::MemberDoesNotExist)
                ,
            }
        }
        /// get member info 
        #[ink(message)]
        pub fn set_dao_admin(&mut self,member_address:AccountId)   {
            
            self.dao_admin = member_address
        }

        
        /// check the caller is the member of dao
        #[ink(message)]
        pub fn is_member(&self,account:AccountId) -> bool {
            let caller = self.env().caller();
            match self.dao_member_list.get(&(account)) {
                Some(_value) => true,
                None => false,
            }
        }
        
        #[inline]
        pub fn _is_member(&self, member_address:AccountId)->bool{
            match self.dao_member_list.get(&(member_address)) {
                Some(_value) => true,
                None => false,
            }
        }
        /// get member list.
        #[ink(message)]
        pub fn get_member_list(&self, dao_address: AccountId) -> Vec<MemberInfo> {
            let mut member_list: Vec<MemberInfo> = Vec::new();
            // let next_member_id = match self.next_member_ids.get(&dao_address) {
            //     Some(value) => value,
            //     None => return member_list,
            // };
            for i in 0..self.next_member_id {
                let member_info = match self.member_infoes_from_id.get(&i) {
                    Some(value) => value,
                    None => continue,
                };
                member_list.push(member_info.clone());
            }
            member_list
        }
        


           /// check the caller is the member of dao
           #[ink(message)]
           pub fn update_member_role(&mut self,dao_address: AccountId,member_address:AccountId,role:MemberRole) -> Result<()> {
            if !self._is_calling_from_dao_manager() {
                return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
            }
            
             let mut member_info =  match self.dao_member_list.get(&(member_address)) {
                   Some(_value) => _value,
                   None => return Err(Error::MemberDoesNotExist),
               };
               let next_member_id = member_info.member_id;
               let mut member_info_ = match self.member_infoes_from_id.get(&next_member_id) {
                Some(value) => value,
                None => {
                    ink_env::debug_println!("MemberDoesNotExist Error.");
                    return Err(Error::MemberDoesNotExist);
                },
                };
               member_info.member_role = role;

               self.dao_member_list
                .insert(&member_address, &member_info.clone());
                 self.member_infoes_from_id
                .insert(&next_member_id, &member_info.clone());
                Self::env().emit_event(MemberRoleUpdated {
                    dao_address: dao_address,
                    member_address: member_address,
                    new_role: member_info.member_role,
                });
               Ok(())
           }

           

           /// create_project
           #[ink(message)]
           pub fn create_project(&mut self,name:String,dao_address: AccountId,assigned_to:AccountId) -> Result<()> {
    
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                if !self._is_member(assigned_to) {
                    return Err(Error::NotAMember);
                }
                // let mut next_project_id = match self.next_project_ids.get(&dao_address) {
                //     Some(value) => value,
                //     None => 0,
                // };
                //calculate the start time 
                let mut task_list:Vec<TaskId> = Vec::new();
                let time_now =  Self::env().block_timestamp();

                // Calculate the timestamp of the seventh day after the original timestamp
                let timestamp_seventh_day = time_now + (7 * SECONDS_PER_DAY);
                let new_sprint = Sprint{
                    project_id:self.next_project_id, 
                    start_date:time_now.to_string(),
                    end_date:  timestamp_seventh_day.to_string() , 
                    action:0
                };
                let project_info = ProjectInfo {
                    name: name,
                    project_id:self.next_project_id,
                    creator:creator,
                    project_status:ProjectStatus::Active,
                    assigned_to:assigned_to,
                    start_time:time_now.to_string(),
                    end_time:None,
                    task_list:task_list,
                    sprint:new_sprint
                };
                let mut  member_info = match self.dao_member_list.get(&(assigned_to)) {
                    Some(value) => value,
                    None => {
                        ink_env::debug_println!("MemberDoesNotExist Error.");
                        return Err(Error::MemberDoesNotExist);
                    },
                };
                member_info.project_list.push(self.next_project_id);
                // self.project_infoes_from_id
                // .insert(&(dao_address, self.next_project_id), &project_info.clone());
                self.dao_project_list
                .insert(&self.next_project_id, &project_info.clone());
              self.next_project_id = self.next_project_id + 1;

                
                Self::env().emit_event(ProjectCreated {
                    dao_address: dao_address,
                    creator: creator,
                    project_id: project_info.project_id,
                    assigned_to:project_info.assigned_to,
                    start_time:project_info.start_time
                });
               Ok(())
           }
           
            /// get_project_info
           #[ink(message)]
           pub fn get_project_info(&mut self,project_id:ProjectId) -> ResultTransaction<ProjectInfo> {
    
            match self.dao_project_list.get(&(project_id)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::ProjectDoesNotExist)
                ,
            }
           }
           
            /// update project status
            #[ink(message)]
            pub fn update_project_status(&mut self,dao_address:AccountId,project_id:ProjectId,status:ProjectStatus) -> Result<()> {
                
            let creator = self.env().caller();
                
            let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };

            if project_info.creator!=creator{
            return Err(Error::NotACreator)
            }

            
            let project_id = project_info.project_id;
            let mut project_info_ = match self.dao_project_list.get(&(project_id)) {
                Some(value) => value,
                None => {
                    ink_env::debug_println!("ProjectDoesNotExist Error.");
                    return Err(Error::ProjectDoesNotExist);
                },
                };
                if status == ProjectStatus::Completed {
                let time_now =  Self::env().block_timestamp();
                project_info.end_time = Some(time_now.to_string());
                }
                 project_info.project_status = status;

                 self.dao_project_list
                .insert(&project_id, &project_info.clone());
                //  self.project_infoes_from_id
                // .insert(&(dao_address, project_id), &project_info.clone());
                Self::env().emit_event(ProjectStatusUpdated {
                    project_id: project_id,
                    status:  project_info.project_status,
                });
                Ok(())

            }
            
            /// close project
            #[ink(message)]
            pub fn close_project(&mut self,dao_address:AccountId,project_id:ProjectId) -> Result<()> {
                
            let creator = self.env().caller();
                
            let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };

            if project_info.creator!=creator{
            return Err(Error::NotACreator)
            }
            let project_id = project_info.project_id;
            let mut project_info_ = match self.dao_project_list.get(&(project_id)) {
                Some(value) => value,
                None => {
                    ink_env::debug_println!("ProjectDoesNotExist Error.");
                    return Err(Error::ProjectDoesNotExist);
                },
                };

            project_info.project_status = ProjectStatus::Completed;
            let time_now =  Self::env().block_timestamp();
            project_info.end_time = Some(time_now.to_string());

            self.dao_project_list
           .insert(&project_id, &project_info.clone());
        //     self.project_infoes_from_id
        //    .insert(&(dao_address, project_id), &project_info.clone());

            Self::env().emit_event(ProjectCompleted {
                project_id: project_id,
                start_time: project_info.start_time,
                end_time: project_info.end_time,
            });
                //TODO

            Ok(())

        }
              
        /// get project list.
        #[ink(message)]
        pub fn get_project_list(&self) -> Vec<ProjectInfo> {
            let mut project_list: Vec<ProjectInfo> = Vec::new();
            // let next_project_id = match self.next_project_ids.get(&dao_address) {
            //     Some(value) => value,
            //     None => return project_list,
            // };
            for i in 0..self.next_project_id {
                let project_info = match self.dao_project_list.get(&i) {
                    Some(value) => value,
                    None => continue,
                };
                project_list.push(project_info.clone());
            }
            project_list
        }   


           /// create_task
           #[ink(message)]
           pub fn create_task(&mut self,name:String,dao_address: AccountId,assigned_to:AccountId,task_type:TaskType,project_id:ProjectId) -> Result<()> {
    
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                if !self._is_member(assigned_to) {
                    return Err(Error::NotAMember);
                }
                let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::ProjectDoesNotExist),
                };
    
                // let mut next_task_id = match self.next_task_ids.get(&(dao_address,project_id)) {
                //     Some(value) => value,
                //     None => 0,
                // };
                
                //calculate the start time 
                let time_now =  Self::env().block_timestamp();
                let task_info = TaskInfo {
                    name: name,
                    task_id:self.next_task_id,
                    project_id:project_id,
                    creator:creator,
                    task_status:TaskStatus::ToDO,
                    assigned_to:assigned_to,
                    start_time:time_now.to_string(),
                    task_type:task_type,
                    end_time:None,
                    review:None,
                    total_time_logged_in:None,
                };
                project_info.task_list.push(self.next_task_id);

                let mut  member_info = match self.dao_member_list.get(&(assigned_to)) {
                    Some(value) => value,
                    None => {
                        ink_env::debug_println!("MemberDoesNotExist Error.");
                        return Err(Error::MemberDoesNotExist);
                    },
                };
                member_info.task_list.push(self.next_task_id);
                self.dao_task_list
                .insert(&self.next_task_id, &task_info.clone());
            self.next_task_id = self.next_task_id + 1;

                // self.next_task_ids.insert(&(dao_address,project_id), &next_task_id);
                //TODO
                // self.project_infoes_from_id
                // .insert(&(dao_address, project_info.project_id), &project_info.clone());
                self.dao_project_list
                .insert(&project_info.project_id, &project_info.clone());
            
                Self::env().emit_event(TaskCreated {
                    task_id: task_info.task_id,
                    project_id:task_info.project_id,
                    creator: task_info.creator,
                    assigned_to:task_info.assigned_to,
                    task_type:task_info.task_type,
                    start_time:task_info.start_time
                });
               Ok(())
           }
           
           /// create_task
           #[ink(message)]
           pub fn create_review(&mut self,dao_address: AccountId,reviewer:AccountId,task_id:TaskId,project_id:ProjectId,review_discription:Vec<u8>,opinion:ReviewOpinion) -> Result<()> {
    
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                if !self._is_member(reviewer) {
                    return Err(Error::NotAMember);
                }
                let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::ProjectDoesNotExist),
                };
                
                let mut task_info = match self.dao_task_list.get(&(task_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };

                let  review = ReviewRecord{
                    who:reviewer,
                    meta:review_discription,
                    option:opinion
                };    

                let  review_status = ReviewStatus{
                    records:review
                };

                task_info.review = Some(review_status);

                
                self.dao_task_list
                .insert(&task_id, &task_info.clone());

                // self.next_task_ids.insert(&(dao_address,project_id), &task_id);
       
               Ok(())
           }

           /// update_task_status
           #[ink(message)]
           pub fn update_task_status(&mut self,dao_address: AccountId,task_id:TaskId,task_status:TaskStatus) -> Result<()> {
    
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                
                let mut task_info = match self.dao_task_list.get(&(task_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };

                if task_info.creator != creator || task_info.assigned_to != creator {
                    return Err(Error::NotaTaskAuthority)
                }
                if task_status == TaskStatus::Completed {
                    let time_now =  Self::env().block_timestamp();
                    task_info.end_time = Some(time_now.to_string());
                }
                task_info.task_status  = task_status;
                let project_id = task_info.project_id;

                
                self.dao_task_list
                .insert(&task_id, &task_info.clone());

       
               Ok(())
           }
           
            /// get_task_info
            #[ink(message)]
            pub fn get_task_info(&mut self,task_id:TaskId) -> ResultTransaction<TaskInfo> {

            match self.dao_task_list.get(&(task_id)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::TaskDoesNotExist)
                ,
            }
            }         
            /// close project
            #[ink(message)]
            pub fn close_task(&mut self,dao_address:AccountId,task_id:TaskId) -> Result<()> {
                
            let creator = self.env().caller();
                
            let mut task_info =  match self.dao_task_list.get(&(task_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };

            if task_info.creator!=creator{
            return Err(Error::NotACreator)
            }
            let project_id = task_info.project_id;
            let mut project_info_ = match self.dao_project_list.get(&(project_id)) {
                Some(value) => value,
                None => {
                    ink_env::debug_println!("ProjectDoesNotExist Error.");
                    return Err(Error::ProjectDoesNotExist);
                },
                };

            task_info.task_status = TaskStatus::Completed;
            let time_now =  Self::env().block_timestamp();
            task_info.end_time = Some(time_now.to_string());
                
            
            self.dao_task_list
                .insert(&task_id, &task_info.clone());                
                //TODO

            Ok(())

            }
             
            /// get project list.
            #[ink(message)]
            pub fn get_task_list(&self, dao_address: AccountId,project_id:ProjectId) -> Vec<TaskInfo> {
                let mut task_list: Vec<TaskInfo> = Vec::new();
                // let next_project_id = match self.next_task_ids.get(&(dao_address,project_id)) {
                //     Some(value) => value,
                //     None => return task_list,
                // };
                for i in 0..self.next_task_id {
                    let project_info = match self.dao_task_list.get(&(i)) {
                        Some(value) => value,
                        None => continue,
                    };
                    task_list.push(project_info.clone());
                }
                task_list
            }   

            #[inline]
            pub fn distribute_reward(&mut self, token_address: AccountId, to_address:AccountId, amount:u128) -> Result<()> {
                    
                let creator = self.env().account_id();

                if !self._is_calling_from_dao_manager() {
                    return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
                }
    
                let token_info: TokenInfo = match self.token_list_for_address.get(&token_address) {
                    Some(value) => value,
                    None => return Err(Error::TheTokenDoesNotExist),
                };
                    
                let mut instance: DaoGovernanceTokenRef = ink_env::call::FromAccountId::from_account_id(token_address);
                
                match instance.distribute_token(to_address ,amount) {
                    Ok(()) => return Ok(()),
                    Err(_e) => return Err(Error::DistributionIsFailure),
                }
                
            }
            /// get project list.
            #[ink(message)]
            pub fn time_log(&mut self, dao_address: AccountId,task_id:TaskId,project_id:ProjectId,time:u16) -> Result<()> {
                let caller = self.env().caller();
                let mut task_info = match self.dao_task_list.get(&(task_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };
                if  task_info.assigned_to != caller {
                    return Err(Error::NotaTaskAuthority)
                }
                task_info.total_time_logged_in  = match task_info.total_time_logged_in{
                        Some(data) =>{
                            Some(data+time)
                        },
                        None =>{
                            Some(0+time)
                        }
                };

                
                self.dao_task_list
                .insert(&task_id, &task_info.clone());

                Ok(())

            }                
            
            #[inline]
            pub fn calculate_efficiency(&mut self, token_address: AccountId, to_address:AccountId, amount:u128) -> Result<()> {
                //TODO
               Ok(())
                
            }   

            // interest_rate: u128, // interest rate per cent
        ///stake duration
        // staking_duration: Timestamp,          
        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get(&self) -> bool {
        //     self.value
        // }

        #[ink(message)]
        pub fn set_stake_data(&mut self, interest_rate: u8, staking_duration:Timestamp) -> Result<()> {
                let caller = self.env().caller();
                if caller != self.dao_admin {
                    return Err(Error::NotAAdmin)
                } 
                self.total_stake = interest_rate;
                self.staking_duration = staking_duration;
           Ok(())
            
        }   

        ///get stake for account 
        #[ink(message)]
        pub fn get_stake_for_account(&self, account_id: AccountId) -> Option<Stake> {
            self.staking_data.get(&account_id)
        }

        ///early withdraw
        #[ink(message)]
        pub fn early_withdraw(&mut self) -> Result<Stake> {
            // Early withdraw don't get any interest
            let account = self.env().caller();
            let stake = self.get_account_if_exists(&account)?;
            let total_amount = stake.staked_amount;

            self.transfer_balance(&account, total_amount)?;
            self.env().emit_event(WithdrawSuccessful {
                staker: account,
                stake: stake.clone(),
            });


            self.staking_data.remove(&account);
            Ok(stake)
        }

        
        
        pub fn check_not_self(&self, account: &AccountId) -> bool {
            self.env().account_id() != *account
        }
        pub fn get_timestamp(&self) -> Timestamp {
            ink_env::block_timestamp::<ink_env::DefaultEnvironment>()
        }

        /// For testing purpose
        #[ink(message)]
        pub fn read_timestamp(&self) -> Option<Timestamp> {
            Some(ink_env::block_timestamp::<ink_env::DefaultEnvironment>())
        }

        pub fn get_account_if_exists(&self, account: &AccountId) -> Result<Stake> {
            if let Some(lock) = self.staking_data.get(account) {
                Ok(lock)
            } else {
                Err(Error::AccountNotExist)
            }
        }

        pub fn check_sufficient_balance(&self, amount: Balance) -> Result<()> {
            if self.env().balance() < amount {
                Err(Error::InsufficientContractBalance)
            } else {
                Ok(())
            }
        }

        pub fn transfer_balance(
            &mut self,
            account: &AccountId,
            balance: Balance,
        ) -> Result<()> {
            self.check_sufficient_balance(balance)?;
            // contract => account 
            // account => contract
            // contract =>  PAN 
            if let Err(_) = self.env().transfer(*account, balance) {
                Err(Error::TransferFailed)
            } else {
                Ok(())
            }
        }
        /// transfer_balance_to_contract
        #[ink(message)]
        #[ink(payable)]
        pub fn transfer_balance_to_contract(
            &mut self,
        ) -> Result<()> {
            let amount = Self::env().transferred_value();   
            ink_env::debug_println!(" elsesese {:#?}",amount);

            Ok(())      
        }
             
        // someone =>  contravt 
        // reddem => contract 
        ///add stake 
        #[ink(message)]
        pub fn add_stake(
            &mut self,
            amount:Balance
        ) -> Result<()> {
            let caller = self.env().caller();

            if self.staking_data.contains(&caller){
                return Err(Error::AlreadyStaked);
            }     
            if self
            .dao_member_list
            .get(&(caller))
            == None 
            {
                if self
                .dao_list
                .get(&(caller))
                == None
                {
                ink_env::debug_println!("########## MEMBER DOES NOT EXISTS .");
                return Err(Error::NotAMember);
                }
            ink_env::debug_println!("########## MEMBER DOES NOT EXISTS .");
            return Err(Error::NotAMember);
            }
                //accountA => amounnt
                //accountB => amount 

            let amount_ = amount/1000000000000;  //10^12 
            self.total_stake = self.total_stake+amount_ as u8;
            // let ownership = self.calculate_ownership(amount_ as u8,self.total_stake);
            let time_now =  Self::env().block_timestamp();
            let mut stake = Stake{
                staked_amount:amount, 
                deposit_time:time_now.to_string(),
                release_time:None, 
            };
            
            self.staking_data.insert(&caller, &stake);
            Self::env().emit_event(DepositSuccessful {
                staker: caller,
                stake: stake,
            });
            Ok(())      
        }

          ///reddem stake
          #[ink(message)]
          pub fn redeem_stake(
              &mut self,
          ) -> Result<()> {
              let caller = self.env().caller();
  
              if self.staking_data.contains(&caller) ==false{
                  return Err(Error::NotStaked);
              }     
              if self
              .dao_member_list
              .get(&(caller))
              == None
              {
                if self
                .dao_list
                .get(&(caller))
                == None
                {
                ink_env::debug_println!("########## MEMBER DOES NOT EXISTS .");
                return Err(Error::NotAMember);
                }
              ink_env::debug_println!("########## MEMBER DOES NOT EXISTS .");
              return Err(Error::NotAMember);
              }
             
              let mut your_stake = match self.staking_data.get(&(caller)) {
                Some(value) => value,
                None => {
                    ink_env::debug_println!("MemberDoesNotExist Error.");
                    return Err(Error::MemberDoesNotExist);
               },
             };
              let staked_amount = your_stake.staked_amount; 
              self.transfer_balance(&caller, staked_amount)?;
              let time_now =  Self::env().block_timestamp();
              your_stake.release_time=Some(time_now.to_string());
              self.total_stake = self.total_stake-staked_amount as u8;
              self.staking_data.remove(&caller);
              Self::env().emit_event(RedeemSuccessful {
                staker: caller,
                stake: your_stake,
            });
              Ok(())      
          }

        #[ink(message)]
        pub fn get_stake_info(&mut self,member_address:AccountId) ->ResultTransaction<Stake> {
            match self.staking_data.get(&(member_address)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::NotStaked)
                ,
            }
        }
        #[ink(message)]
        pub fn get_ownership(&mut self)-> Result<String> {
            let caller = self.env().caller();

          let mut your_stake =  match self.staking_data.get(&(caller)) {
                Some(_value) =>  _value,
                None =>return Err(Error::NotStaked)
                ,
            };
            let staked_amount = your_stake.staked_amount;

            let res = self.calculate_ownership(staked_amount as u8, self.total_stake as u8);

            Ok(res)
        }

        #[inline]
         fn calculate_ownership(&self,   a:u8,
            b:u8,) -> String {
                let a = Fix::from_num(a); 
                let b = Fix::from_num(b); 
                let res = a / b; 
                let res_str =res*100; 
                let s = res_str.to_string();
                ink_env::debug_println!("MemberDoesNotExist Error.{:?}",s);
                s
        }

        #[ink(message)]
       pub fn calculate_ownership_test(&mut self,   amount_:u8,
           )  {
               self.total_stake = self.total_stake+amount_ as u8;
               ink_env::debug_println!("self.total_stake Error.{:?}", self.total_stake);
               let a = Fix::from_num(amount_); 
               let b = Fix::from_num(self.total_stake); 
               let res = a / b; 
               let res_str =res*100; 
               let s = res_str.to_string();
               ink_env::debug_println!("MemberDoesNotExist Error.{:?}",s);
       }

        #[ink(message)]
        pub fn check_contract_balance(
            &mut self,
        ) -> Result<(Balance)> {
            
            
            Ok(self.env().balance() )   
        }


        /*
            fund deposit => contract 
            contract => account 
            custom token => [psp22,psp34....]
            ownership calculations 

            contract owner => 100%
            account A => 1 => 100000000000000 [PAN]
            calculate"_percentage = 1/100 * 100 => 1
            contract owner => 99%
            account B => 1 
            calculate"_percentage = 1/99 * 100 => 0.
            account a => x
            account b => y

            total_stake = x+y+z 
            account a => x/(x+y) 
            account b => y/(x+y)  => 1 = 10^10 
            account c => z/(total_stake)  
                

         */

        // Define a function to calculate the ownership percentage of a member.
         
      
        ///create sprint
        #[ink(message)]
        pub fn create_sprint(&mut self, project_id:ProjectId, start_date:String, end_date:String) -> Result<()>{
            
            let new_sprint = Sprint{
                project_id, 
                start_date,
                end_date, 
                action:0
            };
        let mut project_info_ = match self.dao_project_list.get(&(project_id)) {
            Some(value) => value,
            None => {
                ink_env::debug_println!("ProjectDoesNotExist Error.");
                    return Err(Error::ProjectDoesNotExist);
            },
        };
        project_info_.sprint  = new_sprint; 
        self.dao_project_list
                .insert(&project_id, &project_info_);

        Self::env().emit_event(SprintAdded {
            project_id: project_id,
            start_date:project_info_.sprint.start_date,
            end_date:project_info_.sprint.end_date
        });

        Ok(())
        }



        /// add other dao's 
        #[ink(message)]
        pub fn dao_as_member(&mut self, dao_address: AccountId, dao_info:DaoInfo) -> Result<()> {
            let caller = self.env().caller();
            
            if caller != self.dao_admin {
                return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
            }
            
            if self
            .dao_list
            .get(&(dao_address))
            != None
        {
            ink_env::debug_println!("########## MEMBER EXISTS ALREADY Error.");
            return Err(Error::MemberAlreadyExists);
        }
        
        self.dao_list
        .insert(&dao_address, &dao_info.clone());

        self.next_dao_id = self.next_dao_id +1; 
            Ok(())
        }

        ///get dao list
        #[ink(message)]
        pub fn get_dao_info_with_address(&self,dao_address: AccountId) -> ResultTransaction<DaoInfo>  {
               
            match self.dao_list.get(&(dao_address)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::MemberDoesNotExist)
                ,
            }
        }
        
    }
    

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        // /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let felidaeDAO = FelidaeDao::default();
        //     assert_eq!(felidaeDAO.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut felidaeDAO = FelidaeDao::new(false);
        //     assert_eq!(felidaeDAO.get(), false);
        //     felidaeDAO.flip();
        //     assert_eq!(felidaeDAO.get(), true);
        // }
    }
}
