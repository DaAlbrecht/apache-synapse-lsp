# Send Mediator

The **Send Mediator** is used to send messages out of Synapse to an endpoint. The Send Mediator also copies any message context properties from the current message context to the reply message received on the execution of the send operation, so that the response could be correlated back to the request. Messages may be correlated by WS-A MessageID, or even simple custom text labels.

---

## Info

- The Send mediator is a [content-unaware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.
- A send operation can be blocking or non-blocking depending on the actual transport implementation used. The default NIO-based http/s implementation does not block on a send. Therefore, if a message should be sent and further processed (e.g. transformed) afterwards, it is required to clone the message into two copies and then perform the processing to avoid conflicts.
- Do not add any mediator configurations after Send mediator in the same sequence, because the Micro Integrator does not process them. Any mediator configuration after the Send mediator should go to the outSequence or receive sequence.

## Syntax

``` java
<send/>
```

If the message is to be sent to one or more endpoints, use the following syntax:

``` java
<send>
   (endpointref | endpoint)+
</send>
```

-   The `endpointref` token refers to the following:

    ``` java
    <endpoint key="name"/>
    ```

-   The `endpoint` token refers to an anonymous endpoint definition.

