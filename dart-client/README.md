# kabalist_client
No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

This Dart package is automatically generated by the [OpenAPI Generator](https://openapi-generator.tech) project:

- API version: 0.1.0
- Build package: org.openapitools.codegen.languages.DartClientCodegen

## Requirements

Dart 2.12 or later

## Installation & Usage

### Github
If this Dart package is published to Github, add the following dependency to your pubspec.yaml
```
dependencies:
  kabalist_client:
    git: https://github.com/GIT_USER_ID/GIT_REPO_ID.git
```

### Local
To use the package in your local drive, add the following dependency to your pubspec.yaml
```
dependencies:
  kabalist_client:
    path: /path/to/kabalist_client
```

## Tests

TODO

## Getting Started

Please follow the [installation procedure](#installation--usage) and then run the following:

```dart
import 'package:kabalist_client/api.dart';

// TODO Configure HTTP Bearer authorization: JWT
// Case 1. Use String Token
//defaultApiClient.getAuthentication<HttpBearerAuth>('JWT').setAccessToken('YOUR_ACCESS_TOKEN');
// Case 2. Use Function which generate token.
// String yourTokenGeneratorFunction() { ... }
//defaultApiClient.getAuthentication<HttpBearerAuth>('JWT').setAccessToken(yourTokenGeneratorFunction);

final api_instance = KabaListApi();
final id = 38400000-8cf0-11bd-b23e-10b96e4ef00d; // String | 
final addToListRequest = AddToListRequest(); // AddToListRequest | 

try {
    final result = api_instance.addList(id, addToListRequest);
    print(result);
} catch (e) {
    print('Exception when calling KabaListApi->addList: $e\n');
}

```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*KabaListApi* | [**addList**](doc//KabaListApi.md#addlist) | **POST** /list/{id} | 
*KabaListApi* | [**createList**](doc//KabaListApi.md#createlist) | **POST** /list | 
*KabaListApi* | [**deleteItem**](doc//KabaListApi.md#deleteitem) | **DELETE** /list/{list}/{item} | 
*KabaListApi* | [**deleteList**](doc//KabaListApi.md#deletelist) | **DELETE** /list/{id} | 
*KabaListApi* | [**deleteShares**](doc//KabaListApi.md#deleteshares) | **DELETE** /share/{id} | 
*KabaListApi* | [**getAccountName**](doc//KabaListApi.md#getaccountname) | **GET** /account/{id}/name | 
*KabaListApi* | [**getPublicList**](doc//KabaListApi.md#getpubliclist) | **GET** /public/{id} | 
*KabaListApi* | [**getShares**](doc//KabaListApi.md#getshares) | **GET** /share/{id} | 
*KabaListApi* | [**historySearch**](doc//KabaListApi.md#historysearch) | **GET** /history/{list} | 
*KabaListApi* | [**listLists**](doc//KabaListApi.md#listlists) | **GET** /list | 
*KabaListApi* | [**login**](doc//KabaListApi.md#login) | **POST** /login | 
*KabaListApi* | [**readList**](doc//KabaListApi.md#readlist) | **GET** /list/{id} | 
*KabaListApi* | [**recoverPassword**](doc//KabaListApi.md#recoverpassword) | **POST** /recover/{id} | 
*KabaListApi* | [**recoveryInfo**](doc//KabaListApi.md#recoveryinfo) | **GET** /recover/{id} | 
*KabaListApi* | [**register**](doc//KabaListApi.md#register) | **POST** /register/{id} | 
*KabaListApi* | [**removePublic**](doc//KabaListApi.md#removepublic) | **DELETE** /public/{id} | 
*KabaListApi* | [**searchAccount**](doc//KabaListApi.md#searchaccount) | **GET** /search/account/{name} | 
*KabaListApi* | [**searchList**](doc//KabaListApi.md#searchlist) | **GET** /search/list/{name} | 
*KabaListApi* | [**setPublic**](doc//KabaListApi.md#setpublic) | **PUT** /public/{id} | 
*KabaListApi* | [**shareList**](doc//KabaListApi.md#sharelist) | **PUT** /share/{id} | 
*KabaListApi* | [**unshare**](doc//KabaListApi.md#unshare) | **DELETE** /share/{list}/{account} | 
*KabaListApi* | [**updateItem**](doc//KabaListApi.md#updateitem) | **PATCH** /list/{list}/{item} | 


## Documentation For Models

 - [AddToListRequest](doc//AddToListRequest.md)
 - [AddToListResponse](doc//AddToListResponse.md)
 - [CreateListRequest](doc//CreateListRequest.md)
 - [CreateListResponse](doc//CreateListResponse.md)
 - [ErrResponse](doc//ErrResponse.md)
 - [GetAccountNameResponse](doc//GetAccountNameResponse.md)
 - [GetHistoryResponse](doc//GetHistoryResponse.md)
 - [GetListsResponse](doc//GetListsResponse.md)
 - [GetSharesResponse](doc//GetSharesResponse.md)
 - [Item](doc//Item.md)
 - [ListInfo](doc//ListInfo.md)
 - [ListStatus](doc//ListStatus.md)
 - [LoginRequest](doc//LoginRequest.md)
 - [LoginResponse](doc//LoginResponse.md)
 - [OkResponseForAddToListResponse](doc//OkResponseForAddToListResponse.md)
 - [OkResponseForCreateListResponse](doc//OkResponseForCreateListResponse.md)
 - [OkResponseForEmpty](doc//OkResponseForEmpty.md)
 - [OkResponseForGetAccountNameResponse](doc//OkResponseForGetAccountNameResponse.md)
 - [OkResponseForGetHistoryResponse](doc//OkResponseForGetHistoryResponse.md)
 - [OkResponseForGetListsResponse](doc//OkResponseForGetListsResponse.md)
 - [OkResponseForGetSharesResponse](doc//OkResponseForGetSharesResponse.md)
 - [OkResponseForLoginResponse](doc//OkResponseForLoginResponse.md)
 - [OkResponseForReadListResponse](doc//OkResponseForReadListResponse.md)
 - [OkResponseForRecoveryInfoResponse](doc//OkResponseForRecoveryInfoResponse.md)
 - [OkResponseForSearchAccountResponse](doc//OkResponseForSearchAccountResponse.md)
 - [ReadListResponse](doc//ReadListResponse.md)
 - [RecoverPasswordRequest](doc//RecoverPasswordRequest.md)
 - [RecoveryInfoResponse](doc//RecoveryInfoResponse.md)
 - [RegisterRequest](doc//RegisterRequest.md)
 - [SearchAccountResponse](doc//SearchAccountResponse.md)
 - [ShareListRequest](doc//ShareListRequest.md)
 - [UpdateItemRequest](doc//UpdateItemRequest.md)
 - [UserError](doc//UserError.md)


## Documentation For Authorization


## JWT

- **Type**: HTTP Bearer authentication


## Author


