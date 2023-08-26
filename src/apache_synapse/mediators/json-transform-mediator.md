# JSON Transform Mediator

The **JSON Transform mediator** is used for controlling XML to JSON transformations (possibly with a JSON Schema)  inside a mediation. Normally XML to JSON transformations are controlled by the properties defined in `synapse.properties`. 

Those configurations are applied globally and you cannot have independent configurations for each mediation scenario. 
With JSON Transform mediator you can define the properties inside the mediation and control the transformation independently. 
Also you can have a JSON schema to correct the payload if there are inconsistencies in the transformation.

---

## Info

The JSON Transform mediator is a [content aware](https://apim.docs.wso2.com/en/latest/reference/mediators/about-mediators/#classification-of-mediators) mediator.

## Syntax

``` java
<jsontransform [schema="string"]>
   <property name="string" value="string"/>*
</jsontransform>
```
