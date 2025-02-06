terraform {
  backend "azurerm" {
    resource_group_name  = "bevy_game"
    storage_account_name = "terraformbackendbevygame"
    container_name       = "tfstate"
    key                  = "terraform.tfstate"
    # ARM_CLIENT_ID
    # ARM_CLIENT_SECRET
    # ARM_SUBSCRIPTION_ID
    # ARM_TENANT_ID
  }
}
