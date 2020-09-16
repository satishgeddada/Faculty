<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update assigneed student communication details</description>
   <name>Update assigneed student communication details</name>
   <tag></tag>
   <elementGuidId>b1c48ddf-9987-4a5c-9acd-6b015d86c67e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;communicationData\&quot;:\&quot;{\\\&quot;commuMode\\\&quot;:\\\&quot;Physical Visit\\\&quot;,\\\&quot;commuType\\\&quot;:\\\&quot;Student\\\&quot;,\\\&quot;commuActionItem\\\&quot;:\\\&quot;good\\\&quot;,\\\&quot;commuActionStatus\\\&quot;:\\\&quot;In Progress\\\&quot;,\\\&quot;commuDetails\\\&quot;:\\\&quot;asdasd\\\&quot;,\\\&quot;commuRemarks\\\&quot;:\\\&quot;okay\\\&quot;,\\\&quot;commuDate\\\&quot;:\\\&quot;2020-08-02T18:30:00.000Z\\\&quot;}\&quot;,\&quot;enrollID\&quot;:\&quot;E1A1F3878DA811E9959ABBBF706F3852\&quot;,\&quot;recordStatus\&quot;:\&quot;NotRecordSaved\&quot;,\&quot;options\&quot;:\&quot;ForCommunicationUpdate\&quot;,\&quot;facultyID\&quot;:\&quot;64DF070D8B6B11E98B0925647F276BC6\&quot;,\&quot;commuNo\&quot;:1,\&quot;communDate\&quot;:\&quot;2020-08-03\&quot;}&quot;,
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
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${URL}/faculty/AssignedStudentsServletCommUpdate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.URL</defaultValue>
      <description>url</description>
      <id>bbe6d118-855a-4ea3-be66-4baf19ff4272</id>
      <masked>false</masked>
      <name>URL</name>
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



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
