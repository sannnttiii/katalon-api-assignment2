<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-Create Single Post</name>
   <tag></tag>
   <elementGuidId>d63ca018-1a2c-46a1-b010-9beff4188d5e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;userId\&quot;: ${post_userid},\n  \&quot;id\&quot;:101,\n  \&quot;title\&quot;: \&quot;${post_title}\&quot;,\n  \&quot;body\&quot;: \&quot;${post_body}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>af34a32b-da65-4cf9-ac94-a428ef79d43d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/posts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Global_Post_UserId</defaultValue>
      <description></description>
      <id>d44e35d7-30d3-4b83-9e11-2c7efb49c008</id>
      <masked>false</masked>
      <name>post_userid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Global_Post_Title</defaultValue>
      <description></description>
      <id>cf744bba-3981-42b7-b993-e877854e2ebf</id>
      <masked>false</masked>
      <name>post_title</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Global_Post_Body</defaultValue>
      <description></description>
      <id>c2e10c0d-207a-48ea-af81-f06fdd9642f0</id>
      <masked>false</masked>
      <name>post_body</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'userId', 2)
WS.verifyElementPropertyValue(response, 'id', 101)
WS.verifyElementPropertyValue(response, 'title', 'CREATE NEW DATA')
WS.verifyElementPropertyValue(response, 'body', 'New BODY data')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
