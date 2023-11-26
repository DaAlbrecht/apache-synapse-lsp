# Clone Mediator

The **Clone Mediator** can be used to clone a message into several messages. It resembles the [Scatter-Gather enterprise integration pattern](http://docs.wso2.org/display/IntegrationPatterns/Scatter-Gather). The Clone mediator is similar to the [Iterate mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/iterate-mediator/). The difference between the two mediators is that the Iterate mediator splits a message into different parts, whereas the Clone mediator makes multiple identical copies of the message.

---

## Info

The Clone mediator is a [content-aware](https://apim.docs.wso2.com/en/4.2.0/reference/mediators/about-mediators/#classification-of-mediators) mediator. Also, note that to get the 
asynchronous behavior we have to have the sequence to inject the message context to that sequence asynchronously. We 
can not achieve that by adding the endpoint itself to the target without adding the sequence.

## Syntax

``` java
<clone [continueParent=(true | false)]>
   <target [to="uri"] [soapAction="qname"] [sequence="sequence_ref"] [endpoint="endpoint_ref"]>
     <sequence>
       (mediator)+
     </sequence>?
     <endpoint>
       endpoint
     </endpoint>?
   </target>+
</clone>
```
