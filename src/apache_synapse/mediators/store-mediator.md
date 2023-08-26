# Store Mediator

The **Store mediator** enqueues messages passing through its mediation sequence in a given **message store**. It can serve as a [dead letter channel](https://docs.wso2.com/display/IntegrationPatterns/Dead+Letter+Channel) if it is included in a fault sequence and if its message store is connected to a **message processor** that forwards all the messages in the store to an endpoint.

---

## Syntax

``` java
<axis2ns1:store xmlns:axis2ns1="http://ws.apache.org/ns/synapse" messageStore="JMSMS" sequence="storeSequence"></axis2ns1:store>
```

You can dynamically route messages to a Message Store via the Store mediator by resolving the name of the Message Store from the message context. To enable this, give a path expression (followed by its namespace definitions) for the value of the store name attribute as shown below.

``` java
<axis2ns1:store xmlns:axis2ns1="http://ws.apache.org/ns/synapse" messagestore="{//m:msgstr/m:arg/m:value}"
    xmlns:m="http://services.samples/xsd" sequence="storeSequence"></axis2ns1:store>
```
