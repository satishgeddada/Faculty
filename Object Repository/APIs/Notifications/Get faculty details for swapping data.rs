<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get faculty details for swapping data</description>
   <name>Get faculty details for swapping data</name>
   <tag></tag>
   <elementGuidId>2acfa500-feea-4697-9769-7579b5575d26</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;semesterId\&quot;:\&quot;20208374864487839328\&quot;,\&quot;employeeId\&quot;:\&quot;64DF070D8B6B11E98B0925647F276BC6\&quot;,\&quot;swapdate\&quot;:\&quot;2020-07-28 00:00:00.0\&quot;,\&quot;getstartTime\&quot;:\&quot;2019-10-21 15:50:00.0\&quot;,\&quot;getendTime\&quot;:\&quot;2019-10-21 16:45:00.0\&quot;,\&quot;courseID\&quot;:\&quot;201961010181722142801645\&quot;,\&quot;semsecID\&quot;:\&quot;2020838134583166521353\&quot;,\&quot;dept\&quot;:\&quot;20196101013404918557388\&quot;,\&quot;operation\&quot;:\&quot;forFaculties\&quot;}&quot;,
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
   <restUrl>${URL}/faculty/AttendanceServletforFaculties</restUrl>
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
      <id>015104a0-81e4-42e7-a747-2dcfd9643c37</id>
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
