# Transaction Mediator

A transaction is a set of operations executed as a single unit. It also
can be defined as an agreement, which is carried out between separate
entities or objects. The **Transaction Mediator** is used to manage
**distributed transactions** in the Micro Integrator by providing
transaction functionality for its child mediators.

---

## Info

In addition to distributed transactions, the Micro Integrator also supports Java Message Service (JMS) transactions. For more information on transactions, see [Working with Transactions](https://apim.docs.wso2.com/en/latest/integrate/examples/working-with-transactions/).

## Syntax

```java
<transaction action="commit|fault-if-no-tx|new|rollback|use-existing-or-new"/>
```
