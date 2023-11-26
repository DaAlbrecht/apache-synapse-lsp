# Call Template Mediator

The Call Template mediator allows you to construct a sequence by passing values into a **sequence template**.

---

## Info

This is currently only supported for special types of mediators such as the [Iterator](https://apim.docs.wso2.com/en/latest/reference/mediators/iterate-mediator/) and [Aggregate Mediators](https://apim.docs.wso2.com/en/latest/reference/mediators/aggregate-mediator/), where actual XPath operations are performed on a different SOAP message, and not on the message coming into the mediator.

## Syntax

``` java
<call-template target="string">
   <!-- parameter values will be passed on to a sequence template -->
   (
    <!--passing plain static values -->
   <with-param name="string" value="string" /> |
    <!--passing xpath expressions -->
   <with-param name="string" value="{string}" /> |
    <!--passing dynamic xpath expressions where values will be compiled dynamically-->
   <with-param name="string" value="{{string}}" /> |
   ) *
   <!--this is the in-line sequence of the template    -->
 </call-template>
```

You use the `target` attribute to specify the sequence template you want to use. The `<with-param>` element is used to parse parameter values to the target sequence template. The parameter names should be the same as the names specified in target template. The parameter value can contain a string, an XPath expression (passed in with curly braces { }), or a dynamic XPath expression (passed in with double curly braces) of which the values are compiled dynamically.
