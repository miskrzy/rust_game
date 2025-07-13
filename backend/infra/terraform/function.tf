resource "azurerm_service_plan" "this" {
  name                = format("%s-appserviceplan", local.prefix)
  location            = data.azurerm_resource_group.this.location
  resource_group_name = data.azurerm_resource_group.this.name
  os_type             = "Linux"
  sku_name            = "Y1"
}


resource "azurerm_linux_function_app" "example" {
  name                = format("%s-function", local.prefix)
  resource_group_name = data.azurerm_resource_group.this.name
  location            = data.azurerm_resource_group.this.location

  storage_account_name          = azurerm_storage_account.this.name
  storage_uses_managed_identity = true
  service_plan_id               = azurerm_service_plan.this.id

  site_config {}

  app_settings = {
    FUNCTIONS_WORKER_RUNTIME = "python"
  }

  identity {
    type = "SystemAssigned"
  }
}