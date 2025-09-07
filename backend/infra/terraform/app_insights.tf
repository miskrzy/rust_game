resource "azurerm_application_insights" "this" {
  name                = format("%s-appinsights", local.prefix)
  resource_group_name = data.azurerm_resource_group.this.name
  location            = data.azurerm_resource_group.this.location
  application_type    = "other"

  daily_data_cap_in_gb = 5
  retention_in_days    = 30
}
