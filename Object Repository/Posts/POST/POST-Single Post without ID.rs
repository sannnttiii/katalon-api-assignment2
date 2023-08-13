<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-Single Post without ID</name>
   <tag></tag>
   <elementGuidId>cd517d7c-59fb-4fa3-a91f-66d60b77a2a4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;userId\&quot;: ${post_userid},\n  \&quot;title\&quot;: \&quot;${post_title}\&quot;,\n  \&quot;body\&quot;: \&quot;${post_body}\&quot;\n}&quot;,
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
      <webElementGuid>2b57f8cd-eab3-42b8-b264-f1ca512b0fd2</webElementGuid>
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
      <id>a9aaa177-c72b-4da8-9d40-a77454854434</id>
      <masked>false</masked>
      <name>post_userid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Global_Post_Title</defaultValue>
      <description></description>
      <id>6e49c426-347b-432b-be2b-b20252b77b7e</id>
      <masked>false</masked>
      <name>post_title</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Global_Post_Body</defaultValue>
      <description></description>
      <id>92a3bdc1-38e4-4f9a-a337-397708869981</id>
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
