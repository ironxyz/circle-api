# \PaymentsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_payment**](PaymentsApi.md#cancel_payment) | **POST** /v1/payments/{id}/cancel | Cancel a payment
[**capture_payment**](PaymentsApi.md#capture_payment) | **POST** /v1/payments/{id}/capture | Capture a payment (BETA)
[**create_mock_wire_payment**](PaymentsApi.md#create_mock_wire_payment) | **POST** /v1/mocks/payments/wire | Create a mock Wire payment
[**create_payment**](PaymentsApi.md#create_payment) | **POST** /v1/payments | Create a payment
[**exchange_rate**](PaymentsApi.md#exchange_rate) | **POST** /v1/exchange/quotes | Fetch exchange rate
[**get_payment**](PaymentsApi.md#get_payment) | **GET** /v1/payments/{id} | Get a payment
[**list_payments**](PaymentsApi.md#list_payments) | **GET** /v1/payments | List all payments
[**refund_payment**](PaymentsApi.md#refund_payment) | **POST** /v1/payments/{id}/refund | Refund a payment



## cancel_payment

> models::CancelPaymentResponse cancel_payment(id, cancel_creation_request)
Cancel a payment

The payment will be voided if possible meaning the payment source will not be charged & the payment will never settle. Otherwise, the payment will be refunded meaning the payment source will be charged & the payment will be refunded from deductions of future settlements. Not all payments are eligible to be cancelled.  A successful response does *not* mean the payment has been cancelled; it only means the cancellation request is successfully submitted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**cancel_creation_request** | Option<[**CancelCreationRequest**](CancelCreationRequest.md)> |  |  |

### Return type

[**models::CancelPaymentResponse**](CancelPaymentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## capture_payment

> capture_payment(id, capture_creation_request)
Capture a payment (BETA)

Warning: Please contact Circle support if you are planning on using this feature.  The given amount will be captured for the authorized payment if possible. If no amount is specified, the full amount will be captured. You can only capture once per authorization.  A successful response does *not* mean the payment has been captured. It only means the capture request was successfully submitted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**capture_creation_request** | Option<[**CaptureCreationRequest**](CaptureCreationRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_mock_wire_payment

> models::CreateWirePaymentResponse create_mock_wire_payment(mock_wire_payment_request)
Create a mock Wire payment

In the sandbox environment, initiate a mock wire payment that mimics the behavior of funds sent through the bank (wire) account linked to master wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mock_wire_payment_request** | Option<[**MockWirePaymentRequest**](MockWirePaymentRequest.md)> |  |  |

### Return type

[**models::CreateWirePaymentResponse**](CreateWirePaymentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payment

> models::CreatePaymentResponse create_payment(payment_creation_request)
Create a payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_creation_request** | Option<[**PaymentCreationRequest**](PaymentCreationRequest.md)> |  |  |

### Return type

[**models::CreatePaymentResponse**](CreatePaymentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_rate

> models::FetchExchangeRateResponse exchange_rate(exchange_rate_request)
Fetch exchange rate

Fetches an indicative exchange rate between two currencies. Either the from currency or to currency must be USD.  Note: The current market exchange rate will be applied when Circle receives the deposit. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_rate_request** | Option<[**ExchangeRateRequest**](ExchangeRateRequest.md)> |  |  |

### Return type

[**models::FetchExchangeRateResponse**](FetchExchangeRateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment

> models::GetPaymentResponse get_payment(id)
Get a payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetPaymentResponse**](GetPaymentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payments

> models::ListPaymentsResponse list_payments(source, settlement_id, payment_intent_id, r#type, status, from, to, page_before, page_after, page_size)
List all payments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | Option<**uuid::Uuid**> | Universally unique identifier (UUID v4) for the source. Filters results to fetch only payments made from the provdided source. |  |
**settlement_id** | Option<**uuid::Uuid**> | Queries items with the specified settlement id. Matches any settlement id if unspecified. |  |
**payment_intent_id** | Option<**uuid::Uuid**> | Queries items with the specified payment intent id. |  |
**r#type** | Option<[**Vec<String>**](String.md)> | Source account type. Filters the results to fetch all payments made from a specified account type. Matches any source type if unspecified. |  |
**status** | Option<**String**> | Queries items with the specified status. Matches any status if unspecified. |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListPaymentsResponse**](ListPaymentsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refund_payment

> models::RefundPaymentResponse refund_payment(id, refund_creation_request)
Refund a payment

The payment source will be refunded if possible. Not all payments are eligible to be cancelled.  A successful response does *not* mean the payment has been refunded; it only means the refund request is successfully submitted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**refund_creation_request** | Option<[**RefundCreationRequest**](RefundCreationRequest.md)> |  |  |

### Return type

[**models::RefundPaymentResponse**](RefundPaymentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

