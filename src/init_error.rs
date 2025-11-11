use openvr_sys as sys;
use std::ffi::CStr;
use std::{error, fmt};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum InitError {
    None = sys::EVRInitError_VRInitError_None,
    Unknown = sys::EVRInitError_VRInitError_Unknown,
    Init_InstallationNotFound = sys::EVRInitError_VRInitError_Init_InstallationNotFound,
    Init_InstallationCorrupt = sys::EVRInitError_VRInitError_Init_InstallationCorrupt,
    Init_VRClientDLLNotFound = sys::EVRInitError_VRInitError_Init_VRClientDLLNotFound,
    Init_FileNotFound = sys::EVRInitError_VRInitError_Init_FileNotFound,
    Init_FactoryNotFound = sys::EVRInitError_VRInitError_Init_FactoryNotFound,
    Init_InterfaceNotFound = sys::EVRInitError_VRInitError_Init_InterfaceNotFound,
    Init_InvalidInterface = sys::EVRInitError_VRInitError_Init_InvalidInterface,
    Init_UserConfigDirectoryInvalid = sys::EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid,
    Init_HmdNotFound = sys::EVRInitError_VRInitError_Init_HmdNotFound,
    Init_NotInitialized = sys::EVRInitError_VRInitError_Init_NotInitialized,
    Init_PathRegistryNotFound = sys::EVRInitError_VRInitError_Init_PathRegistryNotFound,
    Init_NoConfigPath = sys::EVRInitError_VRInitError_Init_NoConfigPath,
    Init_NoLogPath = sys::EVRInitError_VRInitError_Init_NoLogPath,
    Init_PathRegistryNotWritable = sys::EVRInitError_VRInitError_Init_PathRegistryNotWritable,
    Init_AppInfoInitFailed = sys::EVRInitError_VRInitError_Init_AppInfoInitFailed,
    Init_Retry = sys::EVRInitError_VRInitError_Init_Retry,
    Init_InitCanceledByUser = sys::EVRInitError_VRInitError_Init_InitCanceledByUser,
    Init_AnotherAppLaunching = sys::EVRInitError_VRInitError_Init_AnotherAppLaunching,
    Init_SettingsInitFailed = sys::EVRInitError_VRInitError_Init_SettingsInitFailed,
    Init_ShuttingDown = sys::EVRInitError_VRInitError_Init_ShuttingDown,
    Init_TooManyObjects = sys::EVRInitError_VRInitError_Init_TooManyObjects,
    Init_NoServerForBackgroundApp = sys::EVRInitError_VRInitError_Init_NoServerForBackgroundApp,
    Init_NotSupportedWithCompositor = sys::EVRInitError_VRInitError_Init_NotSupportedWithCompositor,
    Init_NotAvailableToUtilityApps = sys::EVRInitError_VRInitError_Init_NotAvailableToUtilityApps,
    Init_Internal = sys::EVRInitError_VRInitError_Init_Internal,
    Init_HmdDriverIdIsNone = sys::EVRInitError_VRInitError_Init_HmdDriverIdIsNone,
    Init_HmdNotFoundPresenceFailed = sys::EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed,
    Init_VRMonitorNotFound = sys::EVRInitError_VRInitError_Init_VRMonitorNotFound,
    Init_VRMonitorStartupFailed = sys::EVRInitError_VRInitError_Init_VRMonitorStartupFailed,
    Init_LowPowerWatchdogNotSupported =
        sys::EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported,
    Init_InvalidApplicationType = sys::EVRInitError_VRInitError_Init_InvalidApplicationType,
    Init_NotAvailableToWatchdogApps = sys::EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps,
    Init_WatchdogDisabledInSettings = sys::EVRInitError_VRInitError_Init_WatchdogDisabledInSettings,
    Init_VRDashboardNotFound = sys::EVRInitError_VRInitError_Init_VRDashboardNotFound,
    Init_VRDashboardStartupFailed = sys::EVRInitError_VRInitError_Init_VRDashboardStartupFailed,
    Init_VRHomeNotFound = sys::EVRInitError_VRInitError_Init_VRHomeNotFound,
    Init_VRHomeStartupFailed = sys::EVRInitError_VRInitError_Init_VRHomeStartupFailed,
    Init_RebootingBusy = sys::EVRInitError_VRInitError_Init_RebootingBusy,
    Init_FirmwareUpdateBusy = sys::EVRInitError_VRInitError_Init_FirmwareUpdateBusy,
    Init_FirmwareRecoveryBusy = sys::EVRInitError_VRInitError_Init_FirmwareRecoveryBusy,
    Init_USBServiceBusy = sys::EVRInitError_VRInitError_Init_USBServiceBusy,
    Init_VRWebHelperStartupFailed = sys::EVRInitError_VRInitError_Init_VRWebHelperStartupFailed,
    Init_TrackerManagerInitFailed = sys::EVRInitError_VRInitError_Init_TrackerManagerInitFailed,
    Init_AlreadyRunning = sys::EVRInitError_VRInitError_Init_AlreadyRunning,
    Init_FailedForVrMonitor = sys::EVRInitError_VRInitError_Init_FailedForVrMonitor,
    Init_PropertyManagerInitFailed = sys::EVRInitError_VRInitError_Init_PropertyManagerInitFailed,
    Init_WebServerFailed = sys::EVRInitError_VRInitError_Init_WebServerFailed,
    Init_IllegalTypeTransition = sys::EVRInitError_VRInitError_Init_IllegalTypeTransition,
    Init_MismatchedRuntimes = sys::EVRInitError_VRInitError_Init_MismatchedRuntimes,
    Init_InvalidProcessId = sys::EVRInitError_VRInitError_Init_InvalidProcessId,
    Init_VRServiceStartupFailed = sys::EVRInitError_VRInitError_Init_VRServiceStartupFailed,
    Init_PrismNeedsNewDrivers = sys::EVRInitError_VRInitError_Init_PrismNeedsNewDrivers,
    Init_PrismStartupTimedOut = sys::EVRInitError_VRInitError_Init_PrismStartupTimedOut,
    Init_CouldNotStartPrism = sys::EVRInitError_VRInitError_Init_CouldNotStartPrism,
    Init_PrismClientInitFailed = sys::EVRInitError_VRInitError_Init_PrismClientInitFailed,
    Init_PrismClientStartFailed = sys::EVRInitError_VRInitError_Init_PrismClientStartFailed,
    Init_PrismExitedUnexpectedly = sys::EVRInitError_VRInitError_Init_PrismExitedUnexpectedly,
    Init_BadLuid = sys::EVRInitError_VRInitError_Init_BadLuid,
    Init_NoServerForAppContainer = sys::EVRInitError_VRInitError_Init_NoServerForAppContainer,
    Init_DuplicateBootstrapper = sys::EVRInitError_VRInitError_Init_DuplicateBootstrapper,
    Init_VRDashboardServicePending = sys::EVRInitError_VRInitError_Init_VRDashboardServicePending,
    Init_VRDashboardServiceTimeout = sys::EVRInitError_VRInitError_Init_VRDashboardServiceTimeout,
    Init_VRDashboardServiceStopped = sys::EVRInitError_VRInitError_Init_VRDashboardServiceStopped,
    Init_VRDashboardAlreadyStarted = sys::EVRInitError_VRInitError_Init_VRDashboardAlreadyStarted,
    Init_VRDashboardCopyFailed = sys::EVRInitError_VRInitError_Init_VRDashboardCopyFailed,
    Init_VRDashboardTokenFailure = sys::EVRInitError_VRInitError_Init_VRDashboardTokenFailure,
    Init_VRDashboardEnvironmentFailure =
        sys::EVRInitError_VRInitError_Init_VRDashboardEnvironmentFailure,
    Init_VRDashboardPathFailure = sys::EVRInitError_VRInitError_Init_VRDashboardPathFailure,
    Driver_Failed = sys::EVRInitError_VRInitError_Driver_Failed,
    Driver_Unknown = sys::EVRInitError_VRInitError_Driver_Unknown,
    Driver_HmdUnknown = sys::EVRInitError_VRInitError_Driver_HmdUnknown,
    Driver_NotLoaded = sys::EVRInitError_VRInitError_Driver_NotLoaded,
    Driver_RuntimeOutOfDate = sys::EVRInitError_VRInitError_Driver_RuntimeOutOfDate,
    Driver_HmdInUse = sys::EVRInitError_VRInitError_Driver_HmdInUse,
    Driver_NotCalibrated = sys::EVRInitError_VRInitError_Driver_NotCalibrated,
    Driver_CalibrationInvalid = sys::EVRInitError_VRInitError_Driver_CalibrationInvalid,
    Driver_HmdDisplayNotFound = sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFound,
    Driver_TrackedDeviceInterfaceUnknown =
        sys::EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown,
    Driver_HmdDriverIdOutOfBounds = sys::EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds,
    Driver_HmdDisplayMirrored = sys::EVRInitError_VRInitError_Driver_HmdDisplayMirrored,
    Driver_HmdDisplayNotFoundLaptop = sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFoundLaptop,
    Driver_PeerDriverNotInstalled = sys::EVRInitError_VRInitError_Driver_PeerDriverNotInstalled,
    Driver_WirelessHmdNotConnected = sys::EVRInitError_VRInitError_Driver_WirelessHmdNotConnected,
    IPC_ServerInitFailed = sys::EVRInitError_VRInitError_IPC_ServerInitFailed,
    IPC_ConnectFailed = sys::EVRInitError_VRInitError_IPC_ConnectFailed,
    IPC_SharedStateInitFailed = sys::EVRInitError_VRInitError_IPC_SharedStateInitFailed,
    IPC_CompositorInitFailed = sys::EVRInitError_VRInitError_IPC_CompositorInitFailed,
    IPC_MutexInitFailed = sys::EVRInitError_VRInitError_IPC_MutexInitFailed,
    IPC_Failed = sys::EVRInitError_VRInitError_IPC_Failed,
    IPC_CompositorConnectFailed = sys::EVRInitError_VRInitError_IPC_CompositorConnectFailed,
    IPC_CompositorInvalidConnectResponse =
        sys::EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse,
    IPC_ConnectFailedAfterMultipleAttempts =
        sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts,
    IPC_ConnectFailedAfterTargetExited =
        sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterTargetExited,
    IPC_NamespaceUnavailable = sys::EVRInitError_VRInitError_IPC_NamespaceUnavailable,
    Compositor_Failed = sys::EVRInitError_VRInitError_Compositor_Failed,
    Compositor_D3D11HardwareRequired =
        sys::EVRInitError_VRInitError_Compositor_D3D11HardwareRequired,
    Compositor_FirmwareRequiresUpdate =
        sys::EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate,
    Compositor_OverlayInitFailed = sys::EVRInitError_VRInitError_Compositor_OverlayInitFailed,
    Compositor_ScreenshotsInitFailed =
        sys::EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed,
    Compositor_UnableToCreateDevice = sys::EVRInitError_VRInitError_Compositor_UnableToCreateDevice,
    Compositor_SharedStateIsNull = sys::EVRInitError_VRInitError_Compositor_SharedStateIsNull,
    Compositor_NotificationManagerIsNull =
        sys::EVRInitError_VRInitError_Compositor_NotificationManagerIsNull,
    Compositor_ResourceManagerClientIsNull =
        sys::EVRInitError_VRInitError_Compositor_ResourceManagerClientIsNull,
    Compositor_MessageOverlaySharedStateInitFailure =
        sys::EVRInitError_VRInitError_Compositor_MessageOverlaySharedStateInitFailure,
    Compositor_PropertiesInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_PropertiesInterfaceIsNull,
    Compositor_CreateFullscreenWindowFailed =
        sys::EVRInitError_VRInitError_Compositor_CreateFullscreenWindowFailed,
    Compositor_SettingsInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_SettingsInterfaceIsNull,
    Compositor_FailedToShowWindow = sys::EVRInitError_VRInitError_Compositor_FailedToShowWindow,
    Compositor_DistortInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_DistortInterfaceIsNull,
    Compositor_DisplayFrequencyFailure =
        sys::EVRInitError_VRInitError_Compositor_DisplayFrequencyFailure,
    Compositor_RendererInitializationFailed =
        sys::EVRInitError_VRInitError_Compositor_RendererInitializationFailed,
    Compositor_DXGIFactoryInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_DXGIFactoryInterfaceIsNull,
    Compositor_DXGIFactoryCreateFailed =
        sys::EVRInitError_VRInitError_Compositor_DXGIFactoryCreateFailed,
    Compositor_DXGIFactoryQueryFailed =
        sys::EVRInitError_VRInitError_Compositor_DXGIFactoryQueryFailed,
    Compositor_InvalidAdapterDesktop =
        sys::EVRInitError_VRInitError_Compositor_InvalidAdapterDesktop,
    Compositor_InvalidHmdAttachment = sys::EVRInitError_VRInitError_Compositor_InvalidHmdAttachment,
    Compositor_InvalidOutputDesktop = sys::EVRInitError_VRInitError_Compositor_InvalidOutputDesktop,
    Compositor_InvalidDeviceProvided =
        sys::EVRInitError_VRInitError_Compositor_InvalidDeviceProvided,
    Compositor_D3D11RendererInitializationFailed =
        sys::EVRInitError_VRInitError_Compositor_D3D11RendererInitializationFailed,
    Compositor_FailedToFindDisplayMode =
        sys::EVRInitError_VRInitError_Compositor_FailedToFindDisplayMode,
    Compositor_FailedToCreateSwapChain =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateSwapChain,
    Compositor_FailedToGetBackBuffer =
        sys::EVRInitError_VRInitError_Compositor_FailedToGetBackBuffer,
    Compositor_FailedToCreateRenderTarget =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateRenderTarget,
    Compositor_FailedToCreateDXGI2SwapChain =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2SwapChain,
    Compositor_FailedtoGetDXGI2BackBuffer =
        sys::EVRInitError_VRInitError_Compositor_FailedtoGetDXGI2BackBuffer,
    Compositor_FailedToCreateDXGI2RenderTarget =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2RenderTarget,
    Compositor_FailedToGetDXGIDeviceInterface =
        sys::EVRInitError_VRInitError_Compositor_FailedToGetDXGIDeviceInterface,
    Compositor_SelectDisplayMode = sys::EVRInitError_VRInitError_Compositor_SelectDisplayMode,
    Compositor_FailedToCreateNvAPIRenderTargets =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateNvAPIRenderTargets,
    Compositor_NvAPISetDisplayMode = sys::EVRInitError_VRInitError_Compositor_NvAPISetDisplayMode,
    Compositor_FailedToCreateDirectModeDisplay =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateDirectModeDisplay,
    Compositor_InvalidHmdPropertyContainer =
        sys::EVRInitError_VRInitError_Compositor_InvalidHmdPropertyContainer,
    Compositor_UpdateDisplayFrequency =
        sys::EVRInitError_VRInitError_Compositor_UpdateDisplayFrequency,
    Compositor_CreateRasterizerState =
        sys::EVRInitError_VRInitError_Compositor_CreateRasterizerState,
    Compositor_CreateWireframeRasterizerState =
        sys::EVRInitError_VRInitError_Compositor_CreateWireframeRasterizerState,
    Compositor_CreateSamplerState = sys::EVRInitError_VRInitError_Compositor_CreateSamplerState,
    Compositor_CreateClampToBorderSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateClampToBorderSamplerState,
    Compositor_CreateAnisoSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateAnisoSamplerState,
    Compositor_CreateOverlaySamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlaySamplerState,
    Compositor_CreatePanoramaSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreatePanoramaSamplerState,
    Compositor_CreateFontSamplerState =
        sys::EVRInitError_VRInitError_Compositor_CreateFontSamplerState,
    Compositor_CreateNoBlendState = sys::EVRInitError_VRInitError_Compositor_CreateNoBlendState,
    Compositor_CreateBlendState = sys::EVRInitError_VRInitError_Compositor_CreateBlendState,
    Compositor_CreateAlphaBlendState =
        sys::EVRInitError_VRInitError_Compositor_CreateAlphaBlendState,
    Compositor_CreateBlendStateMaskR =
        sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskR,
    Compositor_CreateBlendStateMaskG =
        sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskG,
    Compositor_CreateBlendStateMaskB =
        sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskB,
    Compositor_CreateDepthStencilState =
        sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilState,
    Compositor_CreateDepthStencilStateNoWrite =
        sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoWrite,
    Compositor_CreateDepthStencilStateNoDepth =
        sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoDepth,
    Compositor_CreateFlushTexture = sys::EVRInitError_VRInitError_Compositor_CreateFlushTexture,
    Compositor_CreateDistortionSurfaces =
        sys::EVRInitError_VRInitError_Compositor_CreateDistortionSurfaces,
    Compositor_CreateConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateConstantBuffer,
    Compositor_CreateHmdPoseConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseConstantBuffer,
    Compositor_CreateHmdPoseStagingConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer,
    Compositor_CreateSharedFrameInfoConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer,
    Compositor_CreateOverlayConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayConstantBuffer,
    Compositor_CreateSceneTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer,
    Compositor_CreateReadableSceneTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer,
    Compositor_CreateLayerGraphicsTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer,
    Compositor_CreateLayerComputeTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer,
    Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer,
    Compositor_CreateComputeHmdPoseConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer,
    Compositor_CreateGeomConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateGeomConstantBuffer,
    Compositor_CreatePanelMaskConstantBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreatePanelMaskConstantBuffer,
    Compositor_CreatePixelSimUBO = sys::EVRInitError_VRInitError_Compositor_CreatePixelSimUBO,
    Compositor_CreateMSAARenderTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateMSAARenderTextures,
    Compositor_CreateResolveRenderTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateResolveRenderTextures,
    Compositor_CreateComputeResolveRenderTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateComputeResolveRenderTextures,
    Compositor_CreateDriverDirectModeResolveTextures =
        sys::EVRInitError_VRInitError_Compositor_CreateDriverDirectModeResolveTextures,
    Compositor_OpenDriverDirectModeResolveTextures =
        sys::EVRInitError_VRInitError_Compositor_OpenDriverDirectModeResolveTextures,
    Compositor_CreateFallbackSyncTexture =
        sys::EVRInitError_VRInitError_Compositor_CreateFallbackSyncTexture,
    Compositor_ShareFallbackSyncTexture =
        sys::EVRInitError_VRInitError_Compositor_ShareFallbackSyncTexture,
    Compositor_CreateOverlayIndexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayIndexBuffer,
    Compositor_CreateOverlayVertexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayVertexBuffer,
    Compositor_CreateTextVertexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateTextVertexBuffer,
    Compositor_CreateTextIndexBuffer =
        sys::EVRInitError_VRInitError_Compositor_CreateTextIndexBuffer,
    Compositor_CreateMirrorTextures = sys::EVRInitError_VRInitError_Compositor_CreateMirrorTextures,
    Compositor_CreateLastFrameRenderTexture =
        sys::EVRInitError_VRInitError_Compositor_CreateLastFrameRenderTexture,
    Compositor_CreateMirrorOverlay = sys::EVRInitError_VRInitError_Compositor_CreateMirrorOverlay,
    Compositor_FailedToCreateVirtualDisplayBackbuffer =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateVirtualDisplayBackbuffer,
    Compositor_DisplayModeNotSupported =
        sys::EVRInitError_VRInitError_Compositor_DisplayModeNotSupported,
    Compositor_CreateOverlayInvalidCall =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayInvalidCall,
    Compositor_CreateOverlayAlreadyInitialized =
        sys::EVRInitError_VRInitError_Compositor_CreateOverlayAlreadyInitialized,
    Compositor_FailedToCreateMailbox =
        sys::EVRInitError_VRInitError_Compositor_FailedToCreateMailbox,
    Compositor_WindowInterfaceIsNull =
        sys::EVRInitError_VRInitError_Compositor_WindowInterfaceIsNull,
    Compositor_SystemLayerCreateInstance =
        sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateInstance,
    Compositor_SystemLayerCreateSession =
        sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateSession,
    Compositor_CreateInverseDistortUVs =
        sys::EVRInitError_VRInitError_Compositor_CreateInverseDistortUVs,
    Compositor_CreateBackbufferDepth =
        sys::EVRInitError_VRInitError_Compositor_CreateBackbufferDepth,
    Compositor_CannotDRMLeaseDisplay =
        sys::EVRInitError_VRInitError_Compositor_CannotDRMLeaseDisplay,
    Compositor_CannotConnectToDisplayServer =
        sys::EVRInitError_VRInitError_Compositor_CannotConnectToDisplayServer,
    Compositor_GnomeNoDRMLeasing = sys::EVRInitError_VRInitError_Compositor_GnomeNoDRMLeasing,
    Compositor_FailedToInitializeEncoder =
        sys::EVRInitError_VRInitError_Compositor_FailedToInitializeEncoder,
    Compositor_CreateBlurTexture = sys::EVRInitError_VRInitError_Compositor_CreateBlurTexture,
    VendorSpecific_UnableToConnectToOculusRuntime =
        sys::EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime,
    VendorSpecific_WindowsNotInDevMode =
        sys::EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode,
    VendorSpecific_OculusLinkNotEnabled =
        sys::EVRInitError_VRInitError_VendorSpecific_OculusLinkNotEnabled,
    VendorSpecific_HmdFound_CantOpenDevice =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice,
    VendorSpecific_HmdFound_UnableToRequestConfigStart =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart,
    VendorSpecific_HmdFound_NoStoredConfig =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig,
    VendorSpecific_HmdFound_ConfigTooBig =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig,
    VendorSpecific_HmdFound_ConfigTooSmall =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall,
    VendorSpecific_HmdFound_UnableToInitZLib =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib,
    VendorSpecific_HmdFound_CantReadFirmwareVersion =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion,
    VendorSpecific_HmdFound_UnableToSendUserDataStart =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart,
    VendorSpecific_HmdFound_UnableToGetUserDataStart =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart,
    VendorSpecific_HmdFound_UnableToGetUserDataNext =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext,
    VendorSpecific_HmdFound_UserDataAddressRange =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange,
    VendorSpecific_HmdFound_UserDataError =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError,
    VendorSpecific_HmdFound_ConfigFailedSanityCheck =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck,
    VendorSpecific_OculusRuntimeBadInstall =
        sys::EVRInitError_VRInitError_VendorSpecific_OculusRuntimeBadInstall,
    VendorSpecific_HmdFound_UnexpectedConfiguration_1 =
        sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnexpectedConfiguration_1,
    Steam_SteamInstallationNotFound = sys::EVRInitError_VRInitError_Steam_SteamInstallationNotFound,
    LastError = sys::EVRInitError_VRInitError_LastError,
}

impl PartialEq<InitError> for &InitError {
    fn eq(&self, other: &InitError) -> bool {
        *self == other
    }
}

impl InitError {
    fn to_sys_err(&self) -> sys::EVRInitError {
        return *self as sys::EVRInitError;
    }
}

impl From<sys::EVRInitError> for InitError {
    fn from(value: sys::EVRInitError) -> Self {
        match value {
            sys::EVRInitError_VRInitError_None => Self::None,
            sys::EVRInitError_VRInitError_Unknown => Self::Unknown,
            sys::EVRInitError_VRInitError_Init_InstallationNotFound => Self::Init_InstallationNotFound,
            sys::EVRInitError_VRInitError_Init_InstallationCorrupt=>Self::Init_InstallationCorrupt,
            sys::EVRInitError_VRInitError_Init_VRClientDLLNotFound=>Self::Init_VRClientDLLNotFound,
            sys::EVRInitError_VRInitError_Init_FileNotFound=>Self::Init_FileNotFound,
            sys::EVRInitError_VRInitError_Init_FactoryNotFound=>Self::Init_FactoryNotFound,
            sys::EVRInitError_VRInitError_Init_InterfaceNotFound=>Self::Init_InterfaceNotFound,
            sys::EVRInitError_VRInitError_Init_InvalidInterface=>Self::Init_InvalidInterface,
            sys::EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid=>Self::Init_UserConfigDirectoryInvalid ,
            sys::EVRInitError_VRInitError_Init_HmdNotFound=>Self::Init_HmdNotFound ,
            sys::EVRInitError_VRInitError_Init_NotInitialized=>Self::Init_NotInitialized ,
            sys::EVRInitError_VRInitError_Init_PathRegistryNotFound=>Self::Init_PathRegistryNotFound ,
            sys::EVRInitError_VRInitError_Init_NoConfigPath=>Self::Init_NoConfigPath ,
            sys::EVRInitError_VRInitError_Init_NoLogPath=>Self::Init_NoLogPath ,
            sys::EVRInitError_VRInitError_Init_PathRegistryNotWritable=>Self::Init_PathRegistryNotWritable ,
            sys::EVRInitError_VRInitError_Init_AppInfoInitFailed=>Self::Init_AppInfoInitFailed ,
            sys::EVRInitError_VRInitError_Init_Retry=>Self::Init_Retry ,
            sys::EVRInitError_VRInitError_Init_InitCanceledByUser=>Self::Init_InitCanceledByUser ,
            sys::EVRInitError_VRInitError_Init_AnotherAppLaunching=>Self::Init_AnotherAppLaunching ,
            sys::EVRInitError_VRInitError_Init_SettingsInitFailed=>Self::Init_SettingsInitFailed ,
            sys::EVRInitError_VRInitError_Init_ShuttingDown=>Self::Init_ShuttingDown ,
            sys::EVRInitError_VRInitError_Init_TooManyObjects=>Self::Init_TooManyObjects ,
            sys::EVRInitError_VRInitError_Init_NoServerForBackgroundApp=>Self::Init_NoServerForBackgroundApp ,
            sys::EVRInitError_VRInitError_Init_NotSupportedWithCompositor=>Self::Init_NotSupportedWithCompositor ,
            sys::EVRInitError_VRInitError_Init_NotAvailableToUtilityApps=>Self::Init_NotAvailableToUtilityApps ,
            sys::EVRInitError_VRInitError_Init_Internal=>Self::Init_Internal ,
            sys::EVRInitError_VRInitError_Init_HmdDriverIdIsNone=>Self::Init_HmdDriverIdIsNone ,
            sys::EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed=>Self::Init_HmdNotFoundPresenceFailed ,
            sys::EVRInitError_VRInitError_Init_VRMonitorNotFound=>Self::Init_VRMonitorNotFound ,
            sys::EVRInitError_VRInitError_Init_VRMonitorStartupFailed=>Self::Init_VRMonitorStartupFailed ,
            sys::EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported=>Self::Init_LowPowerWatchdogNotSupported ,
            sys::EVRInitError_VRInitError_Init_InvalidApplicationType=>Self::Init_InvalidApplicationType ,
            sys::EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps=>Self::Init_NotAvailableToWatchdogApps ,
            sys::EVRInitError_VRInitError_Init_WatchdogDisabledInSettings=>Self::Init_WatchdogDisabledInSettings ,
            sys::EVRInitError_VRInitError_Init_VRDashboardNotFound=>Self::Init_VRDashboardNotFound ,
            sys::EVRInitError_VRInitError_Init_VRDashboardStartupFailed=>Self::Init_VRDashboardStartupFailed ,
            sys::EVRInitError_VRInitError_Init_VRHomeNotFound=>Self::Init_VRHomeNotFound ,
            sys::EVRInitError_VRInitError_Init_VRHomeStartupFailed=>Self::Init_VRHomeStartupFailed ,
            sys::EVRInitError_VRInitError_Init_RebootingBusy=>Self::Init_RebootingBusy ,
            sys::EVRInitError_VRInitError_Init_FirmwareUpdateBusy=>Self::Init_FirmwareUpdateBusy ,
            sys::EVRInitError_VRInitError_Init_FirmwareRecoveryBusy=>Self::Init_FirmwareRecoveryBusy ,
            sys::EVRInitError_VRInitError_Init_USBServiceBusy=>Self::Init_USBServiceBusy ,
            sys::EVRInitError_VRInitError_Init_VRWebHelperStartupFailed=>Self::Init_VRWebHelperStartupFailed ,
            sys::EVRInitError_VRInitError_Init_TrackerManagerInitFailed=>Self::Init_TrackerManagerInitFailed ,
            sys::EVRInitError_VRInitError_Init_AlreadyRunning=>Self::Init_AlreadyRunning ,
            sys::EVRInitError_VRInitError_Init_FailedForVrMonitor=>Self::Init_FailedForVrMonitor ,
            sys::EVRInitError_VRInitError_Init_PropertyManagerInitFailed=>Self::Init_PropertyManagerInitFailed ,
            sys::EVRInitError_VRInitError_Init_WebServerFailed=>Self::Init_WebServerFailed ,
            sys::EVRInitError_VRInitError_Init_IllegalTypeTransition=>Self::Init_IllegalTypeTransition ,
            sys::EVRInitError_VRInitError_Init_MismatchedRuntimes=>Self::Init_MismatchedRuntimes ,
            sys::EVRInitError_VRInitError_Init_InvalidProcessId=>Self::Init_InvalidProcessId ,
            sys::EVRInitError_VRInitError_Init_VRServiceStartupFailed=>Self::Init_VRServiceStartupFailed ,
            sys::EVRInitError_VRInitError_Init_PrismNeedsNewDrivers=>Self::Init_PrismNeedsNewDrivers ,
            sys::EVRInitError_VRInitError_Init_PrismStartupTimedOut=>Self::Init_PrismStartupTimedOut ,
            sys::EVRInitError_VRInitError_Init_CouldNotStartPrism=>Self::Init_CouldNotStartPrism ,
            sys::EVRInitError_VRInitError_Init_PrismClientInitFailed=>Self::Init_PrismClientInitFailed ,
            sys::EVRInitError_VRInitError_Init_PrismClientStartFailed=>Self::Init_PrismClientStartFailed ,
            sys::EVRInitError_VRInitError_Init_PrismExitedUnexpectedly=>Self::Init_PrismExitedUnexpectedly ,
            sys::EVRInitError_VRInitError_Init_BadLuid=>Self::Init_BadLuid ,
            sys::EVRInitError_VRInitError_Init_NoServerForAppContainer=>Self::Init_NoServerForAppContainer ,
            sys::EVRInitError_VRInitError_Init_DuplicateBootstrapper=>Self::Init_DuplicateBootstrapper ,
            sys::EVRInitError_VRInitError_Init_VRDashboardServicePending=>Self::Init_VRDashboardServicePending ,
            sys::EVRInitError_VRInitError_Init_VRDashboardServiceTimeout=>Self::Init_VRDashboardServiceTimeout ,
            sys::EVRInitError_VRInitError_Init_VRDashboardServiceStopped=>Self::Init_VRDashboardServiceStopped ,
            sys::EVRInitError_VRInitError_Init_VRDashboardAlreadyStarted=>Self::Init_VRDashboardAlreadyStarted ,
            sys::EVRInitError_VRInitError_Init_VRDashboardCopyFailed=>Self::Init_VRDashboardCopyFailed ,
            sys::EVRInitError_VRInitError_Init_VRDashboardTokenFailure=>Self::Init_VRDashboardTokenFailure ,
            sys::EVRInitError_VRInitError_Init_VRDashboardEnvironmentFailure=>Self::Init_VRDashboardEnvironmentFailure ,
            sys::EVRInitError_VRInitError_Init_VRDashboardPathFailure=>Self::Init_VRDashboardPathFailure ,
            sys::EVRInitError_VRInitError_Driver_Failed=>Self::Driver_Failed ,
            sys::EVRInitError_VRInitError_Driver_Unknown=>Self::Driver_Unknown ,
            sys::EVRInitError_VRInitError_Driver_HmdUnknown=>Self::Driver_HmdUnknown ,
            sys::EVRInitError_VRInitError_Driver_NotLoaded=>Self::Driver_NotLoaded ,
            sys::EVRInitError_VRInitError_Driver_RuntimeOutOfDate=>Self::Driver_RuntimeOutOfDate ,
            sys::EVRInitError_VRInitError_Driver_HmdInUse=>Self::Driver_HmdInUse ,
            sys::EVRInitError_VRInitError_Driver_NotCalibrated=>Self::Driver_NotCalibrated ,
            sys::EVRInitError_VRInitError_Driver_CalibrationInvalid=>Self::Driver_CalibrationInvalid ,
            sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFound=>Self::Driver_HmdDisplayNotFound ,
            sys::EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown=>Self::Driver_TrackedDeviceInterfaceUnknown ,
            sys::EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds=>Self::Driver_HmdDriverIdOutOfBounds ,
            sys::EVRInitError_VRInitError_Driver_HmdDisplayMirrored=>Self::Driver_HmdDisplayMirrored ,
            sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFoundLaptop=>Self::Driver_HmdDisplayNotFoundLaptop ,
            sys::EVRInitError_VRInitError_Driver_PeerDriverNotInstalled=>Self::Driver_PeerDriverNotInstalled ,
            sys::EVRInitError_VRInitError_Driver_WirelessHmdNotConnected=>Self::Driver_WirelessHmdNotConnected ,
            sys::EVRInitError_VRInitError_IPC_ServerInitFailed=>Self::IPC_ServerInitFailed ,
            sys::EVRInitError_VRInitError_IPC_ConnectFailed=>Self::IPC_ConnectFailed ,
            sys::EVRInitError_VRInitError_IPC_SharedStateInitFailed=>Self::IPC_SharedStateInitFailed ,
            sys::EVRInitError_VRInitError_IPC_CompositorInitFailed=>Self::IPC_CompositorInitFailed ,
            sys::EVRInitError_VRInitError_IPC_MutexInitFailed=>Self::IPC_MutexInitFailed ,
            sys::EVRInitError_VRInitError_IPC_Failed=>Self::IPC_Failed ,
            sys::EVRInitError_VRInitError_IPC_CompositorConnectFailed=>Self::IPC_CompositorConnectFailed ,
            sys::EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse=>Self::IPC_CompositorInvalidConnectResponse ,
            sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts=>Self::IPC_ConnectFailedAfterMultipleAttempts ,
            sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterTargetExited=>Self::IPC_ConnectFailedAfterTargetExited ,
            sys::EVRInitError_VRInitError_IPC_NamespaceUnavailable=>Self::IPC_NamespaceUnavailable ,
            sys::EVRInitError_VRInitError_Compositor_Failed=>Self::Compositor_Failed ,
            sys::EVRInitError_VRInitError_Compositor_D3D11HardwareRequired=>Self::Compositor_D3D11HardwareRequired ,
            sys::EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate=>Self::Compositor_FirmwareRequiresUpdate ,
            sys::EVRInitError_VRInitError_Compositor_OverlayInitFailed=>Self::Compositor_OverlayInitFailed ,
            sys::EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed=>Self::Compositor_ScreenshotsInitFailed ,
            sys::EVRInitError_VRInitError_Compositor_UnableToCreateDevice=>Self::Compositor_UnableToCreateDevice ,
            sys::EVRInitError_VRInitError_Compositor_SharedStateIsNull=>Self::Compositor_SharedStateIsNull ,
            sys::EVRInitError_VRInitError_Compositor_NotificationManagerIsNull=>Self::Compositor_NotificationManagerIsNull ,
            sys::EVRInitError_VRInitError_Compositor_ResourceManagerClientIsNull=>Self::Compositor_ResourceManagerClientIsNull ,
            sys::EVRInitError_VRInitError_Compositor_MessageOverlaySharedStateInitFailure=>Self::Compositor_MessageOverlaySharedStateInitFailure,
            sys::EVRInitError_VRInitError_Compositor_PropertiesInterfaceIsNull=>Self::Compositor_PropertiesInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_CreateFullscreenWindowFailed=>Self::Compositor_CreateFullscreenWindowFailed ,
            sys::EVRInitError_VRInitError_Compositor_SettingsInterfaceIsNull=>Self::Compositor_SettingsInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_FailedToShowWindow=>Self::Compositor_FailedToShowWindow ,
            sys::EVRInitError_VRInitError_Compositor_DistortInterfaceIsNull=>Self::Compositor_DistortInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_DisplayFrequencyFailure=>Self::Compositor_DisplayFrequencyFailure ,
            sys::EVRInitError_VRInitError_Compositor_RendererInitializationFailed=>Self::Compositor_RendererInitializationFailed ,
            sys::EVRInitError_VRInitError_Compositor_DXGIFactoryInterfaceIsNull=>Self::Compositor_DXGIFactoryInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_DXGIFactoryCreateFailed=>Self::Compositor_DXGIFactoryCreateFailed ,
            sys::EVRInitError_VRInitError_Compositor_DXGIFactoryQueryFailed=>Self::Compositor_DXGIFactoryQueryFailed ,
            sys::EVRInitError_VRInitError_Compositor_InvalidAdapterDesktop=>Self::Compositor_InvalidAdapterDesktop ,
            sys::EVRInitError_VRInitError_Compositor_InvalidHmdAttachment=>Self::Compositor_InvalidHmdAttachment ,
            sys::EVRInitError_VRInitError_Compositor_InvalidOutputDesktop=>Self::Compositor_InvalidOutputDesktop ,
            sys::EVRInitError_VRInitError_Compositor_InvalidDeviceProvided=>Self::Compositor_InvalidDeviceProvided ,
            sys::EVRInitError_VRInitError_Compositor_D3D11RendererInitializationFailed=>Self::Compositor_D3D11RendererInitializationFailed ,
            sys::EVRInitError_VRInitError_Compositor_FailedToFindDisplayMode=>Self::Compositor_FailedToFindDisplayMode ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateSwapChain=>Self::Compositor_FailedToCreateSwapChain ,
            sys::EVRInitError_VRInitError_Compositor_FailedToGetBackBuffer=>Self::Compositor_FailedToGetBackBuffer ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateRenderTarget=>Self::Compositor_FailedToCreateRenderTarget ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2SwapChain=>Self::Compositor_FailedToCreateDXGI2SwapChain ,
            sys::EVRInitError_VRInitError_Compositor_FailedtoGetDXGI2BackBuffer=>Self::Compositor_FailedtoGetDXGI2BackBuffer ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2RenderTarget=>Self::Compositor_FailedToCreateDXGI2RenderTarget ,
            sys::EVRInitError_VRInitError_Compositor_FailedToGetDXGIDeviceInterface=>Self::Compositor_FailedToGetDXGIDeviceInterface ,
            sys::EVRInitError_VRInitError_Compositor_SelectDisplayMode=>Self::Compositor_SelectDisplayMode ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateNvAPIRenderTargets=>Self::Compositor_FailedToCreateNvAPIRenderTargets ,
            sys::EVRInitError_VRInitError_Compositor_NvAPISetDisplayMode=>Self::Compositor_NvAPISetDisplayMode ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateDirectModeDisplay=>Self::Compositor_FailedToCreateDirectModeDisplay ,
            sys::EVRInitError_VRInitError_Compositor_InvalidHmdPropertyContainer=>Self::Compositor_InvalidHmdPropertyContainer ,
            sys::EVRInitError_VRInitError_Compositor_UpdateDisplayFrequency=>Self::Compositor_UpdateDisplayFrequency ,
            sys::EVRInitError_VRInitError_Compositor_CreateRasterizerState=>Self::Compositor_CreateRasterizerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateWireframeRasterizerState=>Self::Compositor_CreateWireframeRasterizerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateSamplerState=>Self::Compositor_CreateSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateClampToBorderSamplerState=>Self::Compositor_CreateClampToBorderSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateAnisoSamplerState=>Self::Compositor_CreateAnisoSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlaySamplerState=>Self::Compositor_CreateOverlaySamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreatePanoramaSamplerState=>Self::Compositor_CreatePanoramaSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateFontSamplerState=>Self::Compositor_CreateFontSamplerState ,
            sys::EVRInitError_VRInitError_Compositor_CreateNoBlendState=>Self::Compositor_CreateNoBlendState ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendState=>Self::Compositor_CreateBlendState ,
            sys::EVRInitError_VRInitError_Compositor_CreateAlphaBlendState=>Self::Compositor_CreateAlphaBlendState ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskR=>Self::Compositor_CreateBlendStateMaskR ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskG=>Self::Compositor_CreateBlendStateMaskG ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskB=>Self::Compositor_CreateBlendStateMaskB ,
            sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilState=>Self::Compositor_CreateDepthStencilState ,
            sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoWrite=>Self::Compositor_CreateDepthStencilStateNoWrite ,
            sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoDepth=>Self::Compositor_CreateDepthStencilStateNoDepth ,
            sys::EVRInitError_VRInitError_Compositor_CreateFlushTexture=>Self::Compositor_CreateFlushTexture ,
            sys::EVRInitError_VRInitError_Compositor_CreateDistortionSurfaces=>Self::Compositor_CreateDistortionSurfaces ,
            sys::EVRInitError_VRInitError_Compositor_CreateConstantBuffer=>Self::Compositor_CreateConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseConstantBuffer=>Self::Compositor_CreateHmdPoseConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer=>Self::Compositor_CreateHmdPoseStagingConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer=>Self::Compositor_CreateSharedFrameInfoConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayConstantBuffer=>Self::Compositor_CreateOverlayConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer=>Self::Compositor_CreateSceneTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer=>Self::Compositor_CreateReadableSceneTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer=>Self::Compositor_CreateLayerGraphicsTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer=>Self::Compositor_CreateLayerComputeTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer=>Self::Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer,
            sys::EVRInitError_VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer=>Self::Compositor_CreateComputeHmdPoseConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateGeomConstantBuffer=>Self::Compositor_CreateGeomConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreatePanelMaskConstantBuffer=>Self::Compositor_CreatePanelMaskConstantBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreatePixelSimUBO=>Self::Compositor_CreatePixelSimUBO ,
            sys::EVRInitError_VRInitError_Compositor_CreateMSAARenderTextures=>Self::Compositor_CreateMSAARenderTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateResolveRenderTextures=>Self::Compositor_CreateResolveRenderTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateComputeResolveRenderTextures=>Self::Compositor_CreateComputeResolveRenderTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateDriverDirectModeResolveTextures=>Self::Compositor_CreateDriverDirectModeResolveTextures ,
            sys::EVRInitError_VRInitError_Compositor_OpenDriverDirectModeResolveTextures=>Self::Compositor_OpenDriverDirectModeResolveTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateFallbackSyncTexture=>Self::Compositor_CreateFallbackSyncTexture ,
            sys::EVRInitError_VRInitError_Compositor_ShareFallbackSyncTexture=>Self::Compositor_ShareFallbackSyncTexture ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayIndexBuffer=>Self::Compositor_CreateOverlayIndexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayVertexBuffer=>Self::Compositor_CreateOverlayVertexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateTextVertexBuffer=>Self::Compositor_CreateTextVertexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateTextIndexBuffer=>Self::Compositor_CreateTextIndexBuffer ,
            sys::EVRInitError_VRInitError_Compositor_CreateMirrorTextures=>Self::Compositor_CreateMirrorTextures ,
            sys::EVRInitError_VRInitError_Compositor_CreateLastFrameRenderTexture=>Self::Compositor_CreateLastFrameRenderTexture ,
            sys::EVRInitError_VRInitError_Compositor_CreateMirrorOverlay=>Self::Compositor_CreateMirrorOverlay ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateVirtualDisplayBackbuffer=>Self::Compositor_FailedToCreateVirtualDisplayBackbuffer ,
            sys::EVRInitError_VRInitError_Compositor_DisplayModeNotSupported=>Self::Compositor_DisplayModeNotSupported ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayInvalidCall=>Self::Compositor_CreateOverlayInvalidCall ,
            sys::EVRInitError_VRInitError_Compositor_CreateOverlayAlreadyInitialized=>Self::Compositor_CreateOverlayAlreadyInitialized ,
            sys::EVRInitError_VRInitError_Compositor_FailedToCreateMailbox=>Self::Compositor_FailedToCreateMailbox ,
            sys::EVRInitError_VRInitError_Compositor_WindowInterfaceIsNull=>Self::Compositor_WindowInterfaceIsNull ,
            sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateInstance=>Self::Compositor_SystemLayerCreateInstance ,
            sys::EVRInitError_VRInitError_Compositor_SystemLayerCreateSession=>Self::Compositor_SystemLayerCreateSession ,
            sys::EVRInitError_VRInitError_Compositor_CreateInverseDistortUVs=>Self::Compositor_CreateInverseDistortUVs ,
            sys::EVRInitError_VRInitError_Compositor_CreateBackbufferDepth=>Self::Compositor_CreateBackbufferDepth ,
            sys::EVRInitError_VRInitError_Compositor_CannotDRMLeaseDisplay=>Self::Compositor_CannotDRMLeaseDisplay ,
            sys::EVRInitError_VRInitError_Compositor_CannotConnectToDisplayServer=>Self::Compositor_CannotConnectToDisplayServer ,
            sys::EVRInitError_VRInitError_Compositor_GnomeNoDRMLeasing=>Self::Compositor_GnomeNoDRMLeasing ,
            sys::EVRInitError_VRInitError_Compositor_FailedToInitializeEncoder=>Self::Compositor_FailedToInitializeEncoder ,
            sys::EVRInitError_VRInitError_Compositor_CreateBlurTexture=>Self::Compositor_CreateBlurTexture ,
            sys::EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime=>Self::VendorSpecific_UnableToConnectToOculusRuntime ,
            sys::EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode=>Self::VendorSpecific_WindowsNotInDevMode ,
            sys::EVRInitError_VRInitError_VendorSpecific_OculusLinkNotEnabled=>Self::VendorSpecific_OculusLinkNotEnabled ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice=>Self::VendorSpecific_HmdFound_CantOpenDevice ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart=>Self::VendorSpecific_HmdFound_UnableToRequestConfigStart,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig=>Self::VendorSpecific_HmdFound_NoStoredConfig ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig=>Self::VendorSpecific_HmdFound_ConfigTooBig ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall=>Self::VendorSpecific_HmdFound_ConfigTooSmall ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib=>Self::VendorSpecific_HmdFound_UnableToInitZLib ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion=>Self::VendorSpecific_HmdFound_CantReadFirmwareVersion ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart=>Self::VendorSpecific_HmdFound_UnableToSendUserDataStart ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart=>Self::VendorSpecific_HmdFound_UnableToGetUserDataStart ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext=>Self::VendorSpecific_HmdFound_UnableToGetUserDataNext ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange=>Self::VendorSpecific_HmdFound_UserDataAddressRange ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError=>Self::VendorSpecific_HmdFound_UserDataError ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck=>Self::VendorSpecific_HmdFound_ConfigFailedSanityCheck ,
            sys::EVRInitError_VRInitError_VendorSpecific_OculusRuntimeBadInstall=>Self::VendorSpecific_OculusRuntimeBadInstall ,
            sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnexpectedConfiguration_1=>Self::VendorSpecific_HmdFound_UnexpectedConfiguration_1 ,
            sys::EVRInitError_VRInitError_Steam_SteamInstallationNotFound=>Self::Steam_SteamInstallationNotFound ,
            sys::EVRInitError_VRInitError_LastError=>Self::LastError ,
            _=> Self::Unknown,
        }
    }
}

impl fmt::Debug for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = unsafe { CStr::from_ptr(sys::VR_GetVRInitErrorAsSymbol(self.to_sys_err())) };
        f.pad(msg.to_str().unwrap_or(
            "OpenVR init error description was not valid UTF-8, error description is unavailable.",
        ))
    }
}

impl error::Error for InitError {}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = unsafe {
            CStr::from_ptr(sys::VR_GetVRInitErrorAsEnglishDescription(
                self.to_sys_err(),
            ))
        };
        let description = msg.to_str().unwrap_or(
            "OpenVR init error description was not valid UTF-8, error description is unavailable.",
        );
        f.pad(description)
    }
}
