<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get attendance details</description>
   <name>Get attendance details</name>
   <tag></tag>
   <elementGuidId>1df05f8a-6c35-4206-84d1-a26a204c1c44</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;facultyId\&quot;:\&quot;64DF070D8B6B11E98B0925647F276BC6\&quot;,\&quot;courseId\&quot;:\&quot;201961010181722142801645\&quot;,\&quot;sectionId\&quot;:\&quot;2020838134583166521353\&quot;,\&quot;semesterId\&quot;:\&quot;20208374864487839328\&quot;,\&quot;classDateId\&quot;:\&quot;202083102609051305323\&quot;,\&quot;coursecode\&quot;:\&quot;R17-7G134-A\&quot;,\&quot;operation\&quot;:\&quot;ForStudents\&quot;}&quot;,
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
   <restUrl>${URL}/faculty/AttendanceServletForStudents</restUrl>
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
      <id>ca453b34-07d9-4336-b035-0f7804564867</id>
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
