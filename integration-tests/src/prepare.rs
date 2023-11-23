use async_trait::async_trait;
use integration_utils::{integration_contract::IntegrationContract, misc::ToNear};
use near_workspaces::Account;
use sweat_integration::{SweatFt, FT_CONTRACT};
use sweat_model::{StorageManagementIntegration, SweatApiIntegration};

pub type Context = integration_utils::context::Context<near_workspaces::network::Sandbox>;

#[async_trait]
pub trait IntegrationContext {
    async fn manager(&mut self) -> anyhow::Result<Account>;
    async fn alice(&mut self) -> anyhow::Result<Account>;
    fn ft_contract(&self) -> SweatFt;
}

#[async_trait]
impl IntegrationContext for Context {
    async fn manager(&mut self) -> anyhow::Result<Account> {
        self.account("manager").await
    }

    async fn alice(&mut self) -> anyhow::Result<Account> {
        self.account("alice").await
    }

    fn ft_contract(&self) -> SweatFt {
        SweatFt::with_contract(&self.contracts[FT_CONTRACT])
    }
}

pub async fn prepare_contract() -> anyhow::Result<Context> {
    let mut context = Context::new(&[FT_CONTRACT], "build".into()).await?;
    let manager = context.manager().await?;
    let alice = context.alice().await?;

    context.ft_contract().new(".u.sweat.testnet".to_string().into()).await?;

    context.ft_contract().add_oracle(&manager.to_near()).await?;

    context
        .ft_contract()
        .storage_deposit(alice.to_near().into(), None)
        .await?;

    Ok(context)
}
