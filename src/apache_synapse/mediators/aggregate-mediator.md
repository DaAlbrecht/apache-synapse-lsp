# Aggregate Mediator

The **Aggregate mediator** implements the [Aggregator enterprise integration pattern](https://docs.wso2.com/display/EIP/Aggregator). It
combines (aggregates) the **response messages** of messages that were split by the split by the [Clone](https://apim.docs.wso2.com/en/latest/reference/mediators/clone-mediator/) or
[Iterate](https://apim.docs.wso2.com/en/latest/reference/mediators/iterate-mediator/) mediator. Note that the responses are not necessarily aggregated in the same order that the requests were sent,
even if you set the `equential` attribute to `true` on the Iterate mediator.

---

## Syntax

```java
<aggregate>
   <correlateOn expression="xpath | json-eval(JSON-Path)"/>?
   <completeCondition [timeout="time-in-seconds"]>
     <messageCount min="int-min" max="int-max"/>?
   </completeCondition>?
   <onComplete expression="xpath |  json-eval(JSON-Path)" [sequence="sequence-ref"]>
     (mediator +)?
   </onComplete>
</aggregate>
```
