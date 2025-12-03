# Encryption Tool Usage Guide

This document details the functionality and operational flow of the Encryption Tool, focusing on file handling, encryption, and decryption methods.

## 1. File Input

All cryptographic operations require the selection of a source file via the graphical user interface.

### 1.1. File Selection Procedure

Locate and click the "Open File..." button within the application interface.

A system file dialog will appear. Navigate to the desired source file (.txt, .dat, etc.).

Select the file and confirm the action to load the file contents into the application buffer.

## 2. Encryption Operations

The tool supports two distinct modes for outputting the encrypted data, allowing for either destructive inline modification or non-destructive file creation.

### 2.1. Inline Encryption (Overwrite Mode)

This mode replaces the original file content with the encrypted ciphertext.

| Parameter | Description                                         | Caution                                                                                                              |
|-----------|-----------------------------------------------------|----------------------------------------------------------------------------------------------------------------------|
| Action    | Overwrites the source file loaded in Section 1.     | Destructive. The original plaintext data is permanently lost and replaced. Backup of the source file is recommended. |
| Output    | The original file path contains the encrypted data. |                                                                                                                      |
| Usage     | Select the "Encrypt (Inline)" option.               |                                                                                                                      |




### 2.2. New File Encryption (Non-Destructive Mode)

This mode p reserves the original source file and saves the encrypted data to a new destination.

| Parameter | Description                                             |
|-----------|---------------------------------------------------------|
| Action    | Creates a new file containing the encrypted ciphertext. |
| Output    | The user is prompted for a new file name and location.  |
| Usage     | Select the "Encrypt (New File)" option.                 |

## 3. Decryption Operations

Decryption follows the same dual-mode paradigm as encryption, allowing ciphertext to be returned to plaintext either by overwriting the encrypted file or by writing the plaintext to a new file.

### 3.1. Inline Decryption (Overwrite Mode)

The source file (which must contain ciphertext) is overwritten by the resulting plaintext.


| Parameter | Description                                                        | Caution                                                                 |
|-----------|--------------------------------------------------------------------|-------------------------------------------------------------------------|
| Action    | Overwrites the encrypted source file with the decrypted plaintext. | If this file is still needed as ciphertext, this action is destructive. |
| Output    | The original file path now contains the plaintext data.            |                                                                         |
| Usage     | Select the "Decrypt (Inline)" option.                              |                                                                         |

### 3.2. New File Decryption (Non-Destructive Mode)

This mode preserves the original ciphertext file and outputs the resulting plaintext to a separate file.

| Parameter | Description                                                                     |
|-----------|---------------------------------------------------------------------------------|
| Action    | Creates a new file containing the decrypted plaintext.                          |
| Output    | The user is prompted for a new file name and location for the plaintext output. |
| Usage     | Select the "Decrypt (New File)" option.                                         |