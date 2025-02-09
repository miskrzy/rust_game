resource "azurerm_storage_account" "this" {
  name                = format("%sstorage", local.prefix)
  resource_group_name = data.azurerm_resource_group.this.name
  location            = data.azurerm_resource_group.this.location

  account_tier              = "Standard"
  account_replication_type  = "LRS"
  shared_access_key_enabled = false
}
