# PayloadFactory Mediator

The **PayloadFactory Mediator** transforms or replaces the contents of a
message. That is, you can configure the format of the request or response
and also map it with arguments provided in the payloadfactory configuration.

You can use two methods to format the payload using this mediator.

-   Use the **default** template to write the payload in the required format (JSON, XML, or text).
-   Use the **FreeMarker** template to write the payload. This is particularly useful when 
    defining complex JSON payloads.

You can provide arguments in the mediator configuration to pass values to your payload during runtime.
You can specify a static value or use an XPath/JSON expression to pass values dynamically. 
The values passed by the arguments are evaluated against the existing 
message.

---

## Info

The PayloadFactory mediator is a [content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
<payloadFactory media-type="xml | json" template-type="default | freemarker">
    <format ../>
    <args>       
        <arg (value="string" | expression=" {xpath} | {json} | {text} ")/>* 
    </args> 
</payloadFactory>
```
