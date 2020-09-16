<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>For Updating the WorkExperice</description>
   <name>Updating the Work experience</name>
   <tag></tag>
   <elementGuidId>60bacea8-0842-447d-a348-549946372e4a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${URL}/faculty/saveWorkExperience?workJsonData=%7B%22collegename%22%3A%22AITS%22%2C%22expFrom%22%3A%222018-06-21%22%2C%22clientID%22%3A%227BA54C5313174AE58FCB45B1A40F608A%22%2C%22updatedBy%22%3A%22100%22%2C%22employee%22%3A%7B%22employeePrimaryId%22%3A%225AB4B781DE61438B95F0D20F678088F5%22%2C%22quaternaryprogram%22%3A%22undefined%22%2C%22surName%22%3A%22c%22%2C%22gender%22%3A%22Female%22%2C%22mobileNumber%22%3A%229000567484%22%2C%22aadharNo%22%3A%22123481224301%22%2C%22tertiaryprogram%22%3A%22undefined%22%2C%22firstName%22%3A%22C.%20Suguna%20Devi%22%2C%22secondaryprogram%22%3A%22undefined%22%2C%22quinaryprogram%22%3A%22undefined%22%2C%22password%22%3A%22DEMO051020%22%2C%22employeeType%22%3A%22Teaching%20Staff%22%2C%22alternateMobile%22%3A%227889955220%22%2C%22senaryprogram%22%3A%22undefined%22%2C%22designation</restUrl>
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
      <id>d7d47325-d378-4c45-b3c2-839db7c83c61</id>
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
