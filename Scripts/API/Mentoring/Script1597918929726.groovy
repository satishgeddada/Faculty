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

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the assigned Students data', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get assigned student profile', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Getting the student yearwise marks', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the student coursewise faculty detail', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the student finance details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the student Marks details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the selected student attendance detail', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the selected student communication details onload', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Update assigneed student communication details', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the completed communications count for the selected assigned student', 
        [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the completed communications count for the selected assigned student', 
        [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the completed communication details for the selected student', 
        [('URL') : GlobalVariable.URL]), FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the discrepancy details onload', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the Section students to submit the discrepancy', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Get the discrepancy details of the selected student', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

WS.sendRequestAndVerify(findTestObject('APIs/Notifications/Saving the discrepancy details of the selected student', [('URL') : GlobalVariable.URL]), 
    FailureHandling.CONTINUE_ON_FAILURE)

