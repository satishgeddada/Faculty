import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Profile details', [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Update Personal details', [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Update contact details', [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/get the qualification details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Save Qualification', [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the qualification', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting the Qualification', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the Work experience', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Save Work experience', [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the Work experience', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting the Work experience', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get book publications on load', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving single the book publication', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the single Book publication Record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting the Book publication Record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Download the book publication template', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Uploading the Book Publications filled excel file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the filled book publication data into excel file', 
        [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Inserting the single journal publication', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the journal publication template file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Uploading the paper publication data using the downloaded and filled file', 
        [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the Available Journal publication data as excel file', 
        [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/The Journal publication data on load', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the research guidance on load', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving the single Research record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the single research record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting the single research record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the research template file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Uploading the filled research file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the filled research file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the workshop details onload', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving the single Workshop record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the single workshop record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting the single workshop details record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the workshop template file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Uploading the filled workshop file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the filled Workshoptemplatefile', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the Grants received onload', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving the single grant', [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the single grant', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting single grant record', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the grant template file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Upload the filled grantfile', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the filled grant file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the conference details onload', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving the single conference details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the single conference details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting single conference details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Download Conference template file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Uploading the filled template file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Downloading the filled template file', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the phd details on load', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Add the single phd details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Updating the single phd details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Deleting the phd details', [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

