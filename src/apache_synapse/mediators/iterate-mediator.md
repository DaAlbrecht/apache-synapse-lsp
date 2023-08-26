# Iterate Mediator

The **Iterate Mediator** implements the [Splitter enterprise integration
pattern](http://docs.wso2.org/wiki/display/IntegrationPatterns/Splitter)
and splits the message into a number of different messages derived from
the parent message. The Iterate mediator is similar to the [Clone mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/clone-mediator/). The difference between the two mediators
is, the Iterate mediator splits a message into different parts, whereas the Clone mediator makes multiple identical copies of the message.

---

## Info
-   The Iterate mediator is a [content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.
-   Iterate Mediator is quite similar to the [ForEach mediator](https://apim.docs.wso2.com/en/latest/reference/mediators/foreach-mediator/). You can use complex XPath expressions or JSON expressions to conditionally select elements to iterate over in both mediators. Following are the main difference between ForEach and Iterate mediators:
    -   Use the ForEach mediator only for message transformations. If you
    need to make back-end calls from each iteration, then use the
    iterate mediator.
    -   ForEach supports modifying the original payload. You can use Iterate
    for situations where you send the split messages to a target and
    collect them by an Aggregate in a different flow
    -   You need to always accompany an Iterate with an Aggregate mediator.
    ForEach loops over the sub-messages and merges them back to the same
    parent element of the message.
    -   In Iterate you need to send the split messages to an endpoint to continue the message flow. However, ForEach does not allow using [Call](https://apim.docs.wso2.com/en/4.2.0/reference/mediators/call-mediator), [Send](https://apim.docs.wso2.com/en/4.2.0/reference/mediators/send-mediator) and
    [Callout](https://apim.docs.wso2.com/en/4.2.0/reference/mediators/callout-mediator) mediators in the sequence.
    -   ForEach does not split the message flow, unlike Iterate Mediator. It
    guarantees to execute in the same thread until all iterations are
    complete.

When you use ForEach mediator, you can only loop through segments of the
message and do changes to a particular segment. For example, you can
change the payload using payload factory mediator. But you cannot send
the split message out to a service. Once you exit from the for-each
loop, it automatically aggregates the split segments. This replaces the
for-each function of the complex XSLT mediators using a ForEach mediator
and a Payload Factory mediator. However, to implement the
split-aggregate pattern, you still need to use Iterate mediator.

## Syntax

``` java
<iterate [sequential=(true | false)] [continueParent=(true | false)] [preservePayload=(true | false)] [(attachPath="XPath|json-eval(JSON Path)")? expression="XPath|json-eval(JSON Path)"]>
   <target [to="uri"] [soapAction="qname"] [sequence="sequence_ref"] [endpoint="endpoint_ref"]>
     <sequence>
       (mediator)+
     </sequence>?
     <endpoint>
       endpoint
     </endpoint>?
   </target>+
</iterate>
```
