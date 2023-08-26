# Data Mapper Mediator

Data Mapper mediator is a data mapping solution that can be integrated
into a mediation sequence. It converts and transforms one data format to
another, or changes the structure of the data in a message. It provides
WSO2 Integration Studio with a graphical mapping configuration and
generates the files required for executing this graphical mapping
configuration through the WSO2 Data Mapper engine.

WSO2 Data Mapper is an independent component that does not depend on any
other WSO2 product. However, other products can use the Data Mapper to
achieve/offer data mapping capabilities. Data Mapper Mediator is the
intermediate component, which gives the data mapping
capability into WSO2 Micro Integrator.

Data Mapper mediator finds the configuration files from the Registry and configures the Data Mapper Engine with the input message type (XML/JSON/CSV) and output message type (XML/JSON/CSV). Then it takes the request message from the Micro Integrator message flow and uses the configured Data Mapper Engine to execute the transformation and adds the output message to the Micro Integrator message flow.

---

## Info

The Data Mapper mediator is a [content-aware]({{base_path}}/reference/mediators/about-mediators/#classification-of-mediators) mediator.
If you are running JDK 17 or later NashornJS is not provided by JDK and you need to manually add org.openjdk.nashorn.nashorn-core version 15.3 (https://mvnrepository.com/artifact/org.openjdk.nashorn/nashorn-core/15.3) and org.ow2.asm.asm-util verion 9.3 jars (https://mvnrepository.com/artifact/org.ow2.asm/asm-util/9.2) to <MI_HOME>/wso2/lib directory

## Syntax

```xml
<datamapper config="gov:datamapper/FoodMapping.dmc" inputSchema="gov:datamapper/FoodMapping_inputSchema.json" inputType="XML" outputSchema="gov:datamapper/FoodMapping_outputSchema.json" outputType="XML"/> 
```
