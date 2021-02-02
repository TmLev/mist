# Architecture

## Entities

* `Vm` (VM) reflects virtual machine that runs applications or services.

* `ExternalProvider` (EP) acts as an infrastructure provider for provisioning
  VMs.

* `ServiceProvider` (SP) represents a service provider responsible for serving 
  customer needs. SP can have on-premises infrastructure called 
  "private cloud". In times of high demand

## Interactions

Some entities (mostly, providers) are able to communicate with one another. 
Currently, this is achieved with `stakker`'s `call!` macros.
