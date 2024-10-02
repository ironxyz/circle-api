# \PayoutsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_business_payout**](PayoutsApi.md#create_business_payout) | **POST** /v1/businessAccount/payouts | Create a payout
[**create_payout**](PayoutsApi.md#create_payout) | **POST** /v1/payouts | Create a payout
[**get_business_payout**](PayoutsApi.md#get_business_payout) | **GET** /v1/businessAccount/payouts/{id} | Get a payout
[**get_payout**](PayoutsApi.md#get_payout) | **GET** /v1/payouts/{id} | Get a payout
[**list_business_payouts**](PayoutsApi.md#list_business_payouts) | **GET** /v1/businessAccount/payouts | List all payouts
[**list_net_burn_fee_daily_calculations**](PayoutsApi.md#list_net_burn_fee_daily_calculations) | **GET** /v1/fees/redemption/net/dailyReports | List all NET burn daily fee calculations
[**list_payouts**](PayoutsApi.md#list_payouts) | **GET** /v1/payouts | List all payouts



## create_business_payout

> models::CreateBusinessPayoutResponse create_business_payout(business_payout_creation_request)
Create a payout

 Create a payout.    The following table includes the supported pairs of amount.currency and toAmount.currency for FX payouts:  | amount.currency  | toAmount.currency | | ---------------- | ------------ |  | USD | BRL | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_payout_creation_request** | Option<[**BusinessPayoutCreationRequest**](BusinessPayoutCreationRequest.md)> |  |  |

### Return type

[**models::CreateBusinessPayoutResponse**](CreateBusinessPayoutResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payout

> models::CreatePayoutResponse create_payout(crypto_payout_creation_request)
Create a payout

 Create a crypto payout.    The following table includes the supported pairs of amount.currency and toAmount.currency for address book payouts:  | amount.currency  | toAmount.currency | | ---------------- | ------------      | | USD              | USD               | | USD              | BTC               | | USD              | ETH               | | USD              | MTC               | | EUR              | EUR               | | BTC              | USD               | | BTC              | BTC               | | ETH              | USD               | | ETH              | ETH               | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_payout_creation_request** | Option<[**CryptoPayoutCreationRequest**](CryptoPayoutCreationRequest.md)> |  |  |

### Return type

[**models::CreatePayoutResponse**](CreatePayoutResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_payout

> models::GetBusinessPayoutResponse get_business_payout(id)
Get a payout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetBusinessPayoutResponse**](GetBusinessPayoutResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payout

> models::GetPayoutResponse get_payout(id)
Get a payout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetPayoutResponse**](GetPayoutResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_business_payouts

> models::ListBusinessPayoutsResponse list_business_payouts(destination, r#type, status, from, to, page_before, page_after, page_size)
List all payouts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination** | Option<**uuid::Uuid**> | Universally unique identifier (UUID v4) for the destination bank account. Filters the results to fetch all payouts made to a destination bank account. |  |
**r#type** | Option<**String**> | Destination bank account type. Filters the results to fetch all payouts made to a specified destination bank account type. This query parameter can be passed multiple times to fetch results matching multiple destination bank account types. |  |
**status** | Option<[**Vec<models::PayoutStatus>**](models::PayoutStatus.md)> | Queries items with the specified status. Matches any status if unspecified. |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListBusinessPayoutsResponse**](ListBusinessPayoutsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_net_burn_fee_daily_calculations

> models::ListBurnFeeCalculationsResponse list_net_burn_fee_daily_calculations(minimum_fee_amount, currency, from, to, page_before, page_after, page_size)
List all NET burn daily fee calculations

Searches for NET burn fee daily calculations. This endpoint returns up to 50 daily fee amount calculations in descending chronological order or pageSize, if provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**minimum_fee_amount** | Option<**String**> | Filters out NET burn daily fee calculations below minimumFeeAmount value. |  |
**currency** | Option<**String**> | Queries beneficiary bank account currency. Default is USD. |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListBurnFeeCalculationsResponse**](ListBurnFeeCalculationsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payouts

> models::ListPayoutsResponse list_payouts(source, destination, r#type, status, source_currency, destination_currency, chain, from, to, page_before, page_after, page_size)
List all payouts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source** | Option<**String**> | Identifier for the source wallet. Filters the fetched payout results to only be from a specific source wallet. If not provided, payouts from all wallets will be returned. |  |
**destination** | Option<**uuid::Uuid**> | Universally unique identifier (UUID v4) for the destination. Filters the fetched payout results made to a specific destination. If not provided, payouts to all destinations will be returned. |  |
**r#type** | Option<[**Vec<models::PayoutDestinationType>**](models::PayoutDestinationType.md)> | Destination type. Filters the results to fetch all payouts made to a specified destination type. This query parameter can be passed multiple times to fetch results matching multiple destination types. The address_book destination type cannot be combined with other types. |  |
**status** | Option<[**Vec<models::PayoutStatus>**](models::PayoutStatus.md)> | Queries items with the specified status. Matches any status if unspecified. |  |
**source_currency** | Option<**String**> | Queries items with the specified source currency `amount.currency`. Matches any source currency if unspecified. |  |
**destination_currency** | Option<**String**> | Queries items with the specified destination currency `toAmount.currency`. Matches any destination currency if unspecified. |  |
**chain** | Option<**String**> | Queries items with the specified chain. Matches any chain if unspecified |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListPayoutsResponse**](ListPayoutsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

