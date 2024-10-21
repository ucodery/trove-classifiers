//! Python packaging classifiers as an Enum.
//!
//! trove-classifiers encompass all valid PyPI classifiers, which can be found at
//! https://pypi.org/classifiers/.
//!
//! The exact set of classifiers that is supported is pulled from
//! [trove-classifiers](https://pypi.org/project/trove-classifiers/) which is the
//! canonical source of PyPI's classifiers.
//!
//! Trove classifiers were first defined in
//! [PEP-301](https://peps.python.org/pep-0301/#distutils-trove-classification)
//! and are metadata tags that can be added to python package distributions.
//!
//! Examples
//!
//! ```
//! use std::str::FromStr;
//! use trove_classifiers::Classifier;
//!
//! let possible_classifier = "Development Status :: 5 - Production/Stable";
//!
//! match Classifier::from_str(possible_classifier) {
//!     Ok(classifier) => println!("Yes, {classifier} is a classifier known to pypi.org"),
//!     Err(_) => println!("No, {possible_classifier} is unknown to pypi.org"),
//! }
//! ```

use std::str::Split;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// The version of the python package pypa/trove-classifiers that is captured by Classifier
pub const PYPA_VERSION: &str = "2024.10.16";

/// # Examples
///
/// ```
/// use trove_classifiers::Classifier;
///
/// let license_classifier = Classifier::License__OSIApproved__GNUGeneralPublicLicensev3orlaterGPLv3plus;
/// assert_eq!(license_classifier.as_ref(), "License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)");
/// ```
///
/// ```
/// use std::str::FromStr;
/// use trove_classifiers::Classifier;
///
/// let py3 = Classifier::from_str("Programming Language :: Python :: 3 :: Only")?;
/// assert_eq!(py3, Classifier::ProgrammingLanguage__Python__3__Only);
/// # Ok::<(), strum::ParseError>(())
/// ```
#[derive(AsRefStr, Debug, Display, EnumString, Eq, IntoStaticStr, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Classifier {
    #[strum(serialize = "Development Status :: 1 - Planning")]
    DevelopmentStatus__1Planning,
    #[strum(serialize = "Development Status :: 2 - Pre-Alpha")]
    DevelopmentStatus__2PreAlpha,
    #[strum(serialize = "Development Status :: 3 - Alpha")]
    DevelopmentStatus__3Alpha,
    #[strum(serialize = "Development Status :: 4 - Beta")]
    DevelopmentStatus__4Beta,
    #[strum(serialize = "Development Status :: 5 - Production/Stable")]
    DevelopmentStatus__5ProductionStable,
    #[strum(serialize = "Development Status :: 6 - Mature")]
    DevelopmentStatus__6Mature,
    #[strum(serialize = "Development Status :: 7 - Inactive")]
    DevelopmentStatus__7Inactive,
    #[strum(serialize = "Environment :: Console")]
    Environment__Console,
    #[strum(serialize = "Environment :: Console :: Curses")]
    Environment__Console__Curses,
    #[strum(serialize = "Environment :: Console :: Framebuffer")]
    Environment__Console__Framebuffer,
    #[strum(serialize = "Environment :: Console :: Newt")]
    Environment__Console__Newt,
    #[strum(serialize = "Environment :: Console :: svgalib")]
    Environment__Console__svgalib,
    #[strum(serialize = "Environment :: GPU")]
    Environment__GPU,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA")]
    Environment__GPU__NVIDIACUDA,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 1.0")]
    Environment__GPU__NVIDIACUDA__1_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 1.1")]
    Environment__GPU__NVIDIACUDA__1_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 2.0")]
    Environment__GPU__NVIDIACUDA__2_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 2.1")]
    Environment__GPU__NVIDIACUDA__2_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 2.2")]
    Environment__GPU__NVIDIACUDA__2_2,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 2.3")]
    Environment__GPU__NVIDIACUDA__2_3,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 3.0")]
    Environment__GPU__NVIDIACUDA__3_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 3.1")]
    Environment__GPU__NVIDIACUDA__3_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 3.2")]
    Environment__GPU__NVIDIACUDA__3_2,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 4.0")]
    Environment__GPU__NVIDIACUDA__4_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 4.1")]
    Environment__GPU__NVIDIACUDA__4_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 4.2")]
    Environment__GPU__NVIDIACUDA__4_2,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 5.0")]
    Environment__GPU__NVIDIACUDA__5_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 5.5")]
    Environment__GPU__NVIDIACUDA__5_5,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 6.0")]
    Environment__GPU__NVIDIACUDA__6_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 6.5")]
    Environment__GPU__NVIDIACUDA__6_5,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 7.0")]
    Environment__GPU__NVIDIACUDA__7_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 7.5")]
    Environment__GPU__NVIDIACUDA__7_5,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 8.0")]
    Environment__GPU__NVIDIACUDA__8_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 9.0")]
    Environment__GPU__NVIDIACUDA__9_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 9.1")]
    Environment__GPU__NVIDIACUDA__9_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 9.2")]
    Environment__GPU__NVIDIACUDA__9_2,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 10.0")]
    Environment__GPU__NVIDIACUDA__10_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 10.1")]
    Environment__GPU__NVIDIACUDA__10_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 10.2")]
    Environment__GPU__NVIDIACUDA__10_2,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11")]
    Environment__GPU__NVIDIACUDA__11,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.0")]
    Environment__GPU__NVIDIACUDA__11_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.1")]
    Environment__GPU__NVIDIACUDA__11_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.2")]
    Environment__GPU__NVIDIACUDA__11_2,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.3")]
    Environment__GPU__NVIDIACUDA__11_3,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.4")]
    Environment__GPU__NVIDIACUDA__11_4,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.5")]
    Environment__GPU__NVIDIACUDA__11_5,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.6")]
    Environment__GPU__NVIDIACUDA__11_6,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.7")]
    Environment__GPU__NVIDIACUDA__11_7,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 11.8")]
    Environment__GPU__NVIDIACUDA__11_8,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 12")]
    Environment__GPU__NVIDIACUDA__12,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 12 :: 12.0")]
    Environment__GPU__NVIDIACUDA__12__12_0,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 12 :: 12.1")]
    Environment__GPU__NVIDIACUDA__12__12_1,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 12 :: 12.2")]
    Environment__GPU__NVIDIACUDA__12__12_2,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 12 :: 12.3")]
    Environment__GPU__NVIDIACUDA__12__12_3,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 12 :: 12.4")]
    Environment__GPU__NVIDIACUDA__12__12_4,
    #[strum(serialize = "Environment :: GPU :: NVIDIA CUDA :: 12 :: 12.5")]
    Environment__GPU__NVIDIACUDA__12__12_5,
    #[strum(serialize = "Environment :: Handhelds/PDA's")]
    Environment__HandheldsPDAs,
    #[strum(serialize = "Environment :: MacOS X")]
    Environment__MacOSX,
    #[strum(serialize = "Environment :: MacOS X :: Aqua")]
    Environment__MacOSX__Aqua,
    #[strum(serialize = "Environment :: MacOS X :: Carbon")]
    Environment__MacOSX__Carbon,
    #[strum(serialize = "Environment :: MacOS X :: Cocoa")]
    Environment__MacOSX__Cocoa,
    #[strum(serialize = "Environment :: No Input/Output (Daemon)")]
    Environment__NoInputOutputDaemon,
    #[strum(serialize = "Environment :: OpenStack")]
    Environment__OpenStack,
    #[strum(serialize = "Environment :: Other Environment")]
    Environment__OtherEnvironment,
    #[strum(serialize = "Environment :: Plugins")]
    Environment__Plugins,
    #[strum(serialize = "Environment :: Web Environment")]
    Environment__WebEnvironment,
    #[strum(serialize = "Environment :: Web Environment :: Buffet")]
    Environment__WebEnvironment__Buffet,
    #[strum(serialize = "Environment :: Web Environment :: Mozilla")]
    Environment__WebEnvironment__Mozilla,
    #[strum(serialize = "Environment :: Web Environment :: ToscaWidgets")]
    Environment__WebEnvironment__ToscaWidgets,
    #[strum(serialize = "Environment :: WebAssembly")]
    Environment__WebAssembly,
    #[strum(serialize = "Environment :: WebAssembly :: Emscripten")]
    Environment__WebAssembly__Emscripten,
    #[strum(serialize = "Environment :: WebAssembly :: WASI")]
    Environment__WebAssembly__WASI,
    #[strum(serialize = "Environment :: Win32 (MS Windows)")]
    Environment__Win32MSWindows,
    #[strum(serialize = "Environment :: X11 Applications")]
    Environment__X11Applications,
    #[strum(serialize = "Environment :: X11 Applications :: GTK")]
    Environment__X11Applications__GTK,
    #[strum(serialize = "Environment :: X11 Applications :: Gnome")]
    Environment__X11Applications__Gnome,
    #[strum(serialize = "Environment :: X11 Applications :: KDE")]
    Environment__X11Applications__KDE,
    #[strum(serialize = "Environment :: X11 Applications :: Qt")]
    Environment__X11Applications__Qt,
    #[strum(serialize = "Framework :: AWS CDK")]
    Framework__AWSCDK,
    #[strum(serialize = "Framework :: AWS CDK :: 1")]
    Framework__AWSCDK__1,
    #[strum(serialize = "Framework :: AWS CDK :: 2")]
    Framework__AWSCDK__2,
    #[strum(serialize = "Framework :: AiiDA")]
    Framework__AiiDA,
    #[strum(serialize = "Framework :: Ansible")]
    Framework__Ansible,
    #[strum(serialize = "Framework :: AnyIO")]
    Framework__AnyIO,
    #[strum(serialize = "Framework :: Apache Airflow")]
    Framework__ApacheAirflow,
    #[strum(serialize = "Framework :: Apache Airflow :: Provider")]
    Framework__ApacheAirflow__Provider,
    #[strum(serialize = "Framework :: AsyncIO")]
    Framework__AsyncIO,
    #[strum(serialize = "Framework :: BEAT")]
    Framework__BEAT,
    #[strum(serialize = "Framework :: BFG")]
    Framework__BFG,
    #[strum(serialize = "Framework :: Bob")]
    Framework__Bob,
    #[strum(serialize = "Framework :: Bottle")]
    Framework__Bottle,
    #[strum(serialize = "Framework :: Buildout")]
    Framework__Buildout,
    #[strum(serialize = "Framework :: Buildout :: Extension")]
    Framework__Buildout__Extension,
    #[strum(serialize = "Framework :: Buildout :: Recipe")]
    Framework__Buildout__Recipe,
    #[strum(serialize = "Framework :: CastleCMS")]
    Framework__CastleCMS,
    #[strum(serialize = "Framework :: CastleCMS :: Theme")]
    Framework__CastleCMS__Theme,
    #[strum(serialize = "Framework :: Celery")]
    Framework__Celery,
    #[strum(serialize = "Framework :: Chandler")]
    Framework__Chandler,
    #[strum(serialize = "Framework :: CherryPy")]
    Framework__CherryPy,
    #[strum(serialize = "Framework :: CubicWeb")]
    Framework__CubicWeb,
    #[strum(serialize = "Framework :: Dash")]
    Framework__Dash,
    #[strum(serialize = "Framework :: Datasette")]
    Framework__Datasette,
    #[strum(serialize = "Framework :: Django")]
    Framework__Django,
    #[strum(serialize = "Framework :: Django :: 1")]
    Framework__Django__1,
    #[strum(serialize = "Framework :: Django :: 1.4")]
    Framework__Django__1_4,
    #[strum(serialize = "Framework :: Django :: 1.5")]
    Framework__Django__1_5,
    #[strum(serialize = "Framework :: Django :: 1.6")]
    Framework__Django__1_6,
    #[strum(serialize = "Framework :: Django :: 1.7")]
    Framework__Django__1_7,
    #[strum(serialize = "Framework :: Django :: 1.8")]
    Framework__Django__1_8,
    #[strum(serialize = "Framework :: Django :: 1.9")]
    Framework__Django__1_9,
    #[strum(serialize = "Framework :: Django :: 1.10")]
    Framework__Django__1_10,
    #[strum(serialize = "Framework :: Django :: 1.11")]
    Framework__Django__1_11,
    #[strum(serialize = "Framework :: Django :: 2")]
    Framework__Django__2,
    #[strum(serialize = "Framework :: Django :: 2.0")]
    Framework__Django__2_0,
    #[strum(serialize = "Framework :: Django :: 2.1")]
    Framework__Django__2_1,
    #[strum(serialize = "Framework :: Django :: 2.2")]
    Framework__Django__2_2,
    #[strum(serialize = "Framework :: Django :: 3")]
    Framework__Django__3,
    #[strum(serialize = "Framework :: Django :: 3.0")]
    Framework__Django__3_0,
    #[strum(serialize = "Framework :: Django :: 3.1")]
    Framework__Django__3_1,
    #[strum(serialize = "Framework :: Django :: 3.2")]
    Framework__Django__3_2,
    #[strum(serialize = "Framework :: Django :: 4")]
    Framework__Django__4,
    #[strum(serialize = "Framework :: Django :: 4.0")]
    Framework__Django__4_0,
    #[strum(serialize = "Framework :: Django :: 4.1")]
    Framework__Django__4_1,
    #[strum(serialize = "Framework :: Django :: 4.2")]
    Framework__Django__4_2,
    #[strum(serialize = "Framework :: Django :: 5")]
    Framework__Django__5,
    #[strum(serialize = "Framework :: Django :: 5.0")]
    Framework__Django__5_0,
    #[strum(serialize = "Framework :: Django :: 5.1")]
    Framework__Django__5_1,
    #[strum(serialize = "Framework :: Django :: 5.2")]
    Framework__Django__5_2,
    #[strum(serialize = "Framework :: Django CMS")]
    Framework__DjangoCMS,
    #[strum(serialize = "Framework :: Django CMS :: 3.4")]
    Framework__DjangoCMS__3_4,
    #[strum(serialize = "Framework :: Django CMS :: 3.5")]
    Framework__DjangoCMS__3_5,
    #[strum(serialize = "Framework :: Django CMS :: 3.6")]
    Framework__DjangoCMS__3_6,
    #[strum(serialize = "Framework :: Django CMS :: 3.7")]
    Framework__DjangoCMS__3_7,
    #[strum(serialize = "Framework :: Django CMS :: 3.8")]
    Framework__DjangoCMS__3_8,
    #[strum(serialize = "Framework :: Django CMS :: 3.9")]
    Framework__DjangoCMS__3_9,
    #[strum(serialize = "Framework :: Django CMS :: 3.10")]
    Framework__DjangoCMS__3_10,
    #[strum(serialize = "Framework :: Django CMS :: 3.11")]
    Framework__DjangoCMS__3_11,
    #[strum(serialize = "Framework :: Django CMS :: 4.0")]
    Framework__DjangoCMS__4_0,
    #[strum(serialize = "Framework :: Django CMS :: 4.1")]
    Framework__DjangoCMS__4_1,
    #[strum(serialize = "Framework :: FastAPI")]
    Framework__FastAPI,
    #[strum(serialize = "Framework :: Flake8")]
    Framework__Flake8,
    #[strum(serialize = "Framework :: Flask")]
    Framework__Flask,
    #[strum(serialize = "Framework :: Hatch")]
    Framework__Hatch,
    #[strum(serialize = "Framework :: Hypothesis")]
    Framework__Hypothesis,
    #[strum(serialize = "Framework :: IDLE")]
    Framework__IDLE,
    #[strum(serialize = "Framework :: IPython")]
    Framework__IPython,
    #[strum(serialize = "Framework :: Jupyter")]
    Framework__Jupyter,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab")]
    Framework__Jupyter__JupyterLab,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: 1")]
    Framework__Jupyter__JupyterLab__1,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: 2")]
    Framework__Jupyter__JupyterLab__2,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: 3")]
    Framework__Jupyter__JupyterLab__3,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: 4")]
    Framework__Jupyter__JupyterLab__4,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: Extensions")]
    Framework__Jupyter__JupyterLab__Extensions,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: Extensions :: Mime Renderers")]
    Framework__Jupyter__JupyterLab__Extensions__MimeRenderers,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: Extensions :: Prebuilt")]
    Framework__Jupyter__JupyterLab__Extensions__Prebuilt,
    #[strum(serialize = "Framework :: Jupyter :: JupyterLab :: Extensions :: Themes")]
    Framework__Jupyter__JupyterLab__Extensions__Themes,
    #[strum(serialize = "Framework :: Kedro")]
    Framework__Kedro,
    #[strum(serialize = "Framework :: Lektor")]
    Framework__Lektor,
    #[strum(serialize = "Framework :: Masonite")]
    Framework__Masonite,
    #[strum(serialize = "Framework :: Matplotlib")]
    Framework__Matplotlib,
    #[strum(serialize = "Framework :: MkDocs")]
    Framework__MkDocs,
    #[strum(serialize = "Framework :: Nengo")]
    Framework__Nengo,
    #[strum(serialize = "Framework :: Odoo")]
    Framework__Odoo,
    #[strum(serialize = "Framework :: Odoo :: 8.0")]
    Framework__Odoo__8_0,
    #[strum(serialize = "Framework :: Odoo :: 9.0")]
    Framework__Odoo__9_0,
    #[strum(serialize = "Framework :: Odoo :: 10.0")]
    Framework__Odoo__10_0,
    #[strum(serialize = "Framework :: Odoo :: 11.0")]
    Framework__Odoo__11_0,
    #[strum(serialize = "Framework :: Odoo :: 12.0")]
    Framework__Odoo__12_0,
    #[strum(serialize = "Framework :: Odoo :: 13.0")]
    Framework__Odoo__13_0,
    #[strum(serialize = "Framework :: Odoo :: 14.0")]
    Framework__Odoo__14_0,
    #[strum(serialize = "Framework :: Odoo :: 15.0")]
    Framework__Odoo__15_0,
    #[strum(serialize = "Framework :: Odoo :: 16.0")]
    Framework__Odoo__16_0,
    #[strum(serialize = "Framework :: Odoo :: 17.0")]
    Framework__Odoo__17_0,
    #[strum(serialize = "Framework :: Odoo :: 18.0")]
    Framework__Odoo__18_0,
    #[strum(serialize = "Framework :: OpenTelemetry")]
    Framework__OpenTelemetry,
    #[strum(serialize = "Framework :: OpenTelemetry :: Distros")]
    Framework__OpenTelemetry__Distros,
    #[strum(serialize = "Framework :: OpenTelemetry :: Exporters")]
    Framework__OpenTelemetry__Exporters,
    #[strum(serialize = "Framework :: OpenTelemetry :: Instrumentations")]
    Framework__OpenTelemetry__Instrumentations,
    #[strum(serialize = "Framework :: Opps")]
    Framework__Opps,
    #[strum(serialize = "Framework :: Paste")]
    Framework__Paste,
    #[strum(serialize = "Framework :: Pelican")]
    Framework__Pelican,
    #[strum(serialize = "Framework :: Pelican :: Plugins")]
    Framework__Pelican__Plugins,
    #[strum(serialize = "Framework :: Pelican :: Themes")]
    Framework__Pelican__Themes,
    #[strum(serialize = "Framework :: Plone")]
    Framework__Plone,
    #[strum(serialize = "Framework :: Plone :: 3.2")]
    Framework__Plone__3_2,
    #[strum(serialize = "Framework :: Plone :: 3.3")]
    Framework__Plone__3_3,
    #[strum(serialize = "Framework :: Plone :: 4.0")]
    Framework__Plone__4_0,
    #[strum(serialize = "Framework :: Plone :: 4.1")]
    Framework__Plone__4_1,
    #[strum(serialize = "Framework :: Plone :: 4.2")]
    Framework__Plone__4_2,
    #[strum(serialize = "Framework :: Plone :: 4.3")]
    Framework__Plone__4_3,
    #[strum(serialize = "Framework :: Plone :: 5.0")]
    Framework__Plone__5_0,
    #[strum(serialize = "Framework :: Plone :: 5.1")]
    Framework__Plone__5_1,
    #[strum(serialize = "Framework :: Plone :: 5.2")]
    Framework__Plone__5_2,
    #[strum(serialize = "Framework :: Plone :: 5.3")]
    Framework__Plone__5_3,
    #[strum(serialize = "Framework :: Plone :: 6.0")]
    Framework__Plone__6_0,
    #[strum(serialize = "Framework :: Plone :: 6.1")]
    Framework__Plone__6_1,
    #[strum(serialize = "Framework :: Plone :: Addon")]
    Framework__Plone__Addon,
    #[strum(serialize = "Framework :: Plone :: Core")]
    Framework__Plone__Core,
    #[strum(serialize = "Framework :: Plone :: Distribution")]
    Framework__Plone__Distribution,
    #[strum(serialize = "Framework :: Plone :: Theme")]
    Framework__Plone__Theme,
    #[strum(serialize = "Framework :: PySimpleGUI")]
    Framework__PySimpleGUI,
    #[strum(serialize = "Framework :: PySimpleGUI :: 4")]
    Framework__PySimpleGUI__4,
    #[strum(serialize = "Framework :: PySimpleGUI :: 5")]
    Framework__PySimpleGUI__5,
    #[strum(serialize = "Framework :: Pycsou")]
    Framework__Pycsou,
    #[strum(serialize = "Framework :: Pydantic")]
    Framework__Pydantic,
    #[strum(serialize = "Framework :: Pydantic :: 1")]
    Framework__Pydantic__1,
    #[strum(serialize = "Framework :: Pydantic :: 2")]
    Framework__Pydantic__2,
    #[strum(serialize = "Framework :: Pylons")]
    Framework__Pylons,
    #[strum(serialize = "Framework :: Pyramid")]
    Framework__Pyramid,
    #[strum(serialize = "Framework :: Pytest")]
    Framework__Pytest,
    #[strum(serialize = "Framework :: Review Board")]
    Framework__ReviewBoard,
    #[strum(serialize = "Framework :: Robot Framework")]
    Framework__RobotFramework,
    #[strum(serialize = "Framework :: Robot Framework :: Library")]
    Framework__RobotFramework__Library,
    #[strum(serialize = "Framework :: Robot Framework :: Tool")]
    Framework__RobotFramework__Tool,
    #[strum(serialize = "Framework :: Scrapy")]
    Framework__Scrapy,
    #[strum(serialize = "Framework :: Setuptools Plugin")]
    Framework__SetuptoolsPlugin,
    #[strum(serialize = "Framework :: Sphinx")]
    Framework__Sphinx,
    #[strum(serialize = "Framework :: Sphinx :: Domain")]
    Framework__Sphinx__Domain,
    #[strum(serialize = "Framework :: Sphinx :: Extension")]
    Framework__Sphinx__Extension,
    #[strum(serialize = "Framework :: Sphinx :: Theme")]
    Framework__Sphinx__Theme,
    #[strum(serialize = "Framework :: Trac")]
    Framework__Trac,
    #[strum(serialize = "Framework :: Trio")]
    Framework__Trio,
    #[strum(serialize = "Framework :: Tryton")]
    Framework__Tryton,
    #[strum(serialize = "Framework :: TurboGears")]
    Framework__TurboGears,
    #[strum(serialize = "Framework :: TurboGears :: Applications")]
    Framework__TurboGears__Applications,
    #[strum(serialize = "Framework :: TurboGears :: Widgets")]
    Framework__TurboGears__Widgets,
    #[strum(serialize = "Framework :: Twisted")]
    Framework__Twisted,
    #[strum(serialize = "Framework :: Wagtail")]
    Framework__Wagtail,
    #[strum(serialize = "Framework :: Wagtail :: 1")]
    Framework__Wagtail__1,
    #[strum(serialize = "Framework :: Wagtail :: 2")]
    Framework__Wagtail__2,
    #[strum(serialize = "Framework :: Wagtail :: 3")]
    Framework__Wagtail__3,
    #[strum(serialize = "Framework :: Wagtail :: 4")]
    Framework__Wagtail__4,
    #[strum(serialize = "Framework :: Wagtail :: 5")]
    Framework__Wagtail__5,
    #[strum(serialize = "Framework :: Wagtail :: 6")]
    Framework__Wagtail__6,
    #[strum(serialize = "Framework :: ZODB")]
    Framework__ZODB,
    #[strum(serialize = "Framework :: Zope")]
    Framework__Zope,
    #[strum(serialize = "Framework :: Zope2")]
    Framework__Zope2,
    #[strum(serialize = "Framework :: Zope3")]
    Framework__Zope3,
    #[strum(serialize = "Framework :: Zope :: 2")]
    Framework__Zope__2,
    #[strum(serialize = "Framework :: Zope :: 3")]
    Framework__Zope__3,
    #[strum(serialize = "Framework :: Zope :: 4")]
    Framework__Zope__4,
    #[strum(serialize = "Framework :: Zope :: 5")]
    Framework__Zope__5,
    #[strum(serialize = "Framework :: aiohttp")]
    Framework__aiohttp,
    #[strum(serialize = "Framework :: cocotb")]
    Framework__cocotb,
    #[strum(serialize = "Framework :: napari")]
    Framework__napari,
    #[strum(serialize = "Framework :: tox")]
    Framework__tox,
    #[strum(serialize = "Intended Audience :: Customer Service")]
    IntendedAudience__CustomerService,
    #[strum(serialize = "Intended Audience :: Developers")]
    IntendedAudience__Developers,
    #[strum(serialize = "Intended Audience :: Education")]
    IntendedAudience__Education,
    #[strum(serialize = "Intended Audience :: End Users/Desktop")]
    IntendedAudience__EndUsersDesktop,
    #[strum(serialize = "Intended Audience :: Financial and Insurance Industry")]
    IntendedAudience__FinancialandInsuranceIndustry,
    #[strum(serialize = "Intended Audience :: Healthcare Industry")]
    IntendedAudience__HealthcareIndustry,
    #[strum(serialize = "Intended Audience :: Information Technology")]
    IntendedAudience__InformationTechnology,
    #[strum(serialize = "Intended Audience :: Legal Industry")]
    IntendedAudience__LegalIndustry,
    #[strum(serialize = "Intended Audience :: Manufacturing")]
    IntendedAudience__Manufacturing,
    #[strum(serialize = "Intended Audience :: Other Audience")]
    IntendedAudience__OtherAudience,
    #[strum(serialize = "Intended Audience :: Religion")]
    IntendedAudience__Religion,
    #[strum(serialize = "Intended Audience :: Science/Research")]
    IntendedAudience__ScienceResearch,
    #[strum(serialize = "Intended Audience :: System Administrators")]
    IntendedAudience__SystemAdministrators,
    #[strum(serialize = "Intended Audience :: Telecommunications Industry")]
    IntendedAudience__TelecommunicationsIndustry,
    #[strum(serialize = "License :: Aladdin Free Public License (AFPL)")]
    License__AladdinFreePublicLicenseAFPL,
    #[strum(serialize = "License :: CC0 1.0 Universal (CC0 1.0) Public Domain Dedication")]
    License__CC01_0UniversalCC01_0PublicDomainDedication,
    #[strum(serialize = "License :: CeCILL-B Free Software License Agreement (CECILL-B)")]
    License__CeCILLBFreeSoftwareLicenseAgreementCECILLB,
    #[strum(serialize = "License :: CeCILL-C Free Software License Agreement (CECILL-C)")]
    License__CeCILLCFreeSoftwareLicenseAgreementCECILLC,
    #[strum(serialize = "License :: DFSG approved")]
    License__DFSGapproved,
    #[strum(serialize = "License :: Eiffel Forum License (EFL)")]
    License__EiffelForumLicenseEFL,
    #[strum(serialize = "License :: Free For Educational Use")]
    License__FreeForEducationalUse,
    #[strum(serialize = "License :: Free For Home Use")]
    License__FreeForHomeUse,
    #[strum(serialize = "License :: Free To Use But Restricted")]
    License__FreeToUseButRestricted,
    #[strum(serialize = "License :: Free for non-commercial use")]
    License__Freefornoncommercialuse,
    #[strum(serialize = "License :: Freely Distributable")]
    License__FreelyDistributable,
    #[strum(serialize = "License :: Freeware")]
    License__Freeware,
    #[strum(serialize = "License :: GUST Font License 1.0")]
    License__GUSTFontLicense1_0,
    #[strum(serialize = "License :: GUST Font License 2006-09-30")]
    License__GUSTFontLicense20060930,
    #[strum(serialize = "License :: Netscape Public License (NPL)")]
    License__NetscapePublicLicenseNPL,
    #[strum(serialize = "License :: Nokia Open Source License (NOKOS)")]
    License__NokiaOpenSourceLicenseNOKOS,
    #[strum(serialize = "License :: OSI Approved")]
    License__OSIApproved,
    #[strum(serialize = "License :: OSI Approved :: Academic Free License (AFL)")]
    License__OSIApproved__AcademicFreeLicenseAFL,
    #[strum(serialize = "License :: OSI Approved :: Apache Software License")]
    License__OSIApproved__ApacheSoftwareLicense,
    #[strum(serialize = "License :: OSI Approved :: Apple Public Source License")]
    License__OSIApproved__ApplePublicSourceLicense,
    #[strum(serialize = "License :: OSI Approved :: Artistic License")]
    License__OSIApproved__ArtisticLicense,
    #[strum(serialize = "License :: OSI Approved :: Attribution Assurance License")]
    License__OSIApproved__AttributionAssuranceLicense,
    #[strum(serialize = "License :: OSI Approved :: BSD License")]
    License__OSIApproved__BSDLicense,
    #[strum(serialize = "License :: OSI Approved :: Blue Oak Model License (BlueOak-1.0.0)")]
    License__OSIApproved__BlueOakModelLicenseBlueOak1_0_0,
    #[strum(serialize = "License :: OSI Approved :: Boost Software License 1.0 (BSL-1.0)")]
    License__OSIApproved__BoostSoftwareLicense1_0BSL1_0,
    #[strum(
        serialize = "License :: OSI Approved :: CEA CNRS Inria Logiciel Libre License, version 2.1 (CeCILL-2.1)"
    )]
    License__OSIApproved__CEACNRSInriaLogicielLibreLicense,
    version2_1CeCILL2_1,
    #[strum(serialize = "License :: OSI Approved :: CMU License (MIT-CMU)")]
    License__OSIApproved__CMULicenseMITCMU,
    #[strum(
        serialize = "License :: OSI Approved :: Common Development and Distribution License 1.0 (CDDL-1.0)"
    )]
    License__OSIApproved__CommonDevelopmentandDistributionLicense1_0CDDL1_0,
    #[strum(serialize = "License :: OSI Approved :: Common Public License")]
    License__OSIApproved__CommonPublicLicense,
    #[strum(serialize = "License :: OSI Approved :: Eclipse Public License 1.0 (EPL-1.0)")]
    License__OSIApproved__EclipsePublicLicense1_0EPL1_0,
    #[strum(serialize = "License :: OSI Approved :: Eclipse Public License 2.0 (EPL-2.0)")]
    License__OSIApproved__EclipsePublicLicense2_0EPL2_0,
    #[strum(
        serialize = "License :: OSI Approved :: Educational Community License, Version 2.0 (ECL-2.0)"
    )]
    License__OSIApproved__EducationalCommunityLicense,
    Version2_0ECL2_0,
    #[strum(serialize = "License :: OSI Approved :: Eiffel Forum License")]
    License__OSIApproved__EiffelForumLicense,
    #[strum(serialize = "License :: OSI Approved :: European Union Public Licence 1.0 (EUPL 1.0)")]
    License__OSIApproved__EuropeanUnionPublicLicence1_0EUPL1_0,
    #[strum(serialize = "License :: OSI Approved :: European Union Public Licence 1.1 (EUPL 1.1)")]
    License__OSIApproved__EuropeanUnionPublicLicence1_1EUPL1_1,
    #[strum(serialize = "License :: OSI Approved :: European Union Public Licence 1.2 (EUPL 1.2)")]
    License__OSIApproved__EuropeanUnionPublicLicence1_2EUPL1_2,
    #[strum(serialize = "License :: OSI Approved :: GNU Affero General Public License v3")]
    License__OSIApproved__GNUAfferoGeneralPublicLicensev3,
    #[strum(
        serialize = "License :: OSI Approved :: GNU Affero General Public License v3 or later (AGPLv3+)"
    )]
    License__OSIApproved__GNUAfferoGeneralPublicLicensev3orlaterAGPLv3plus,
    #[strum(serialize = "License :: OSI Approved :: GNU Free Documentation License (FDL)")]
    License__OSIApproved__GNUFreeDocumentationLicenseFDL,
    #[strum(serialize = "License :: OSI Approved :: GNU General Public License (GPL)")]
    License__OSIApproved__GNUGeneralPublicLicenseGPL,
    #[strum(serialize = "License :: OSI Approved :: GNU General Public License v2 (GPLv2)")]
    License__OSIApproved__GNUGeneralPublicLicensev2GPLv2,
    #[strum(
        serialize = "License :: OSI Approved :: GNU General Public License v2 or later (GPLv2+)"
    )]
    License__OSIApproved__GNUGeneralPublicLicensev2orlaterGPLv2plus,
    #[strum(serialize = "License :: OSI Approved :: GNU General Public License v3 (GPLv3)")]
    License__OSIApproved__GNUGeneralPublicLicensev3GPLv3,
    #[strum(
        serialize = "License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)"
    )]
    License__OSIApproved__GNUGeneralPublicLicensev3orlaterGPLv3plus,
    #[strum(
        serialize = "License :: OSI Approved :: GNU Lesser General Public License v2 (LGPLv2)"
    )]
    License__OSIApproved__GNULesserGeneralPublicLicensev2LGPLv2,
    #[strum(
        serialize = "License :: OSI Approved :: GNU Lesser General Public License v2 or later (LGPLv2+)"
    )]
    License__OSIApproved__GNULesserGeneralPublicLicensev2orlaterLGPLv2plus,
    #[strum(
        serialize = "License :: OSI Approved :: GNU Lesser General Public License v3 (LGPLv3)"
    )]
    License__OSIApproved__GNULesserGeneralPublicLicensev3LGPLv3,
    #[strum(
        serialize = "License :: OSI Approved :: GNU Lesser General Public License v3 or later (LGPLv3+)"
    )]
    License__OSIApproved__GNULesserGeneralPublicLicensev3orlaterLGPLv3plus,
    #[strum(
        serialize = "License :: OSI Approved :: GNU Library or Lesser General Public License (LGPL)"
    )]
    License__OSIApproved__GNULibraryorLesserGeneralPublicLicenseLGPL,
    #[strum(
        serialize = "License :: OSI Approved :: Historical Permission Notice and Disclaimer (HPND)"
    )]
    License__OSIApproved__HistoricalPermissionNoticeandDisclaimerHPND,
    #[strum(serialize = "License :: OSI Approved :: IBM Public License")]
    License__OSIApproved__IBMPublicLicense,
    #[strum(serialize = "License :: OSI Approved :: ISC License (ISCL)")]
    License__OSIApproved__ISCLicenseISCL,
    #[strum(serialize = "License :: OSI Approved :: Intel Open Source License")]
    License__OSIApproved__IntelOpenSourceLicense,
    #[strum(serialize = "License :: OSI Approved :: Jabber Open Source License")]
    License__OSIApproved__JabberOpenSourceLicense,
    #[strum(serialize = "License :: OSI Approved :: MIT License")]
    License__OSIApproved__MITLicense,
    #[strum(serialize = "License :: OSI Approved :: MIT No Attribution License (MIT-0)")]
    License__OSIApproved__MITNoAttributionLicenseMIT0,
    #[strum(
        serialize = "License :: OSI Approved :: MITRE Collaborative Virtual Workspace License (CVW)"
    )]
    License__OSIApproved__MITRECollaborativeVirtualWorkspaceLicenseCVW,
    #[strum(serialize = "License :: OSI Approved :: MirOS License (MirOS)")]
    License__OSIApproved__MirOSLicenseMirOS,
    #[strum(serialize = "License :: OSI Approved :: Motosoto License")]
    License__OSIApproved__MotosotoLicense,
    #[strum(serialize = "License :: OSI Approved :: Mozilla Public License 1.0 (MPL)")]
    License__OSIApproved__MozillaPublicLicense1_0MPL,
    #[strum(serialize = "License :: OSI Approved :: Mozilla Public License 1.1 (MPL 1.1)")]
    License__OSIApproved__MozillaPublicLicense1_1MPL1_1,
    #[strum(serialize = "License :: OSI Approved :: Mozilla Public License 2.0 (MPL 2.0)")]
    License__OSIApproved__MozillaPublicLicense2_0MPL2_0,
    #[strum(
        serialize = "License :: OSI Approved :: Mulan Permissive Software License v2 (MulanPSL-2.0)"
    )]
    License__OSIApproved__MulanPermissiveSoftwareLicensev2MulanPSL2_0,
    #[strum(serialize = "License :: OSI Approved :: NASA Open Source Agreement v1.3 (NASA-1.3)")]
    License__OSIApproved__NASAOpenSourceAgreementv1_3NASA1_3,
    #[strum(serialize = "License :: OSI Approved :: Nethack General Public License")]
    License__OSIApproved__NethackGeneralPublicLicense,
    #[strum(serialize = "License :: OSI Approved :: Nokia Open Source License")]
    License__OSIApproved__NokiaOpenSourceLicense,
    #[strum(serialize = "License :: OSI Approved :: Open Group Test Suite License")]
    License__OSIApproved__OpenGroupTestSuiteLicense,
    #[strum(serialize = "License :: OSI Approved :: Open Software License 3.0 (OSL-3.0)")]
    License__OSIApproved__OpenSoftwareLicense3_0OSL3_0,
    #[strum(serialize = "License :: OSI Approved :: PostgreSQL License")]
    License__OSIApproved__PostgreSQLLicense,
    #[strum(serialize = "License :: OSI Approved :: Python License (CNRI Python License)")]
    License__OSIApproved__PythonLicenseCNRIPythonLicense,
    #[strum(serialize = "License :: OSI Approved :: Python Software Foundation License")]
    License__OSIApproved__PythonSoftwareFoundationLicense,
    #[strum(serialize = "License :: OSI Approved :: Qt Public License (QPL)")]
    License__OSIApproved__QtPublicLicenseQPL,
    #[strum(serialize = "License :: OSI Approved :: Ricoh Source Code Public License")]
    License__OSIApproved__RicohSourceCodePublicLicense,
    #[strum(serialize = "License :: OSI Approved :: SIL Open Font License 1.1 (OFL-1.1)")]
    License__OSIApproved__SILOpenFontLicense1_1OFL1_1,
    #[strum(serialize = "License :: OSI Approved :: Sleepycat License")]
    License__OSIApproved__SleepycatLicense,
    #[strum(
        serialize = "License :: OSI Approved :: Sun Industry Standards Source License (SISSL)"
    )]
    License__OSIApproved__SunIndustryStandardsSourceLicenseSISSL,
    #[strum(serialize = "License :: OSI Approved :: Sun Public License")]
    License__OSIApproved__SunPublicLicense,
    #[strum(serialize = "License :: OSI Approved :: The Unlicense (Unlicense)")]
    License__OSIApproved__TheUnlicenseUnlicense,
    #[strum(serialize = "License :: OSI Approved :: Universal Permissive License (UPL)")]
    License__OSIApproved__UniversalPermissiveLicenseUPL,
    #[strum(
        serialize = "License :: OSI Approved :: University of Illinois/NCSA Open Source License"
    )]
    License__OSIApproved__UniversityofIllinoisNCSAOpenSourceLicense,
    #[strum(serialize = "License :: OSI Approved :: Vovida Software License 1.0")]
    License__OSIApproved__VovidaSoftwareLicense1_0,
    #[strum(serialize = "License :: OSI Approved :: W3C License")]
    License__OSIApproved__W3CLicense,
    #[strum(serialize = "License :: OSI Approved :: X.Net License")]
    License__OSIApproved__X_NetLicense,
    #[strum(serialize = "License :: OSI Approved :: Zero-Clause BSD (0BSD)")]
    License__OSIApproved__ZeroClauseBSD0BSD,
    #[strum(serialize = "License :: OSI Approved :: Zope Public License")]
    License__OSIApproved__ZopePublicLicense,
    #[strum(serialize = "License :: OSI Approved :: zlib/libpng License")]
    License__OSIApproved__zliblibpngLicense,
    #[strum(serialize = "License :: Other/Proprietary License")]
    License__OtherProprietaryLicense,
    #[strum(serialize = "License :: Public Domain")]
    License__PublicDomain,
    #[strum(serialize = "License :: Repoze Public License")]
    License__RepozePublicLicense,
    #[strum(serialize = "Natural Language :: Afrikaans")]
    NaturalLanguage__Afrikaans,
    #[strum(serialize = "Natural Language :: Arabic")]
    NaturalLanguage__Arabic,
    #[strum(serialize = "Natural Language :: Basque")]
    NaturalLanguage__Basque,
    #[strum(serialize = "Natural Language :: Bengali")]
    NaturalLanguage__Bengali,
    #[strum(serialize = "Natural Language :: Bosnian")]
    NaturalLanguage__Bosnian,
    #[strum(serialize = "Natural Language :: Bulgarian")]
    NaturalLanguage__Bulgarian,
    #[strum(serialize = "Natural Language :: Cantonese")]
    NaturalLanguage__Cantonese,
    #[strum(serialize = "Natural Language :: Catalan")]
    NaturalLanguage__Catalan,
    #[strum(serialize = "Natural Language :: Catalan (Valencian)")]
    NaturalLanguage__CatalanValencian,
    #[strum(serialize = "Natural Language :: Chinese (Simplified)")]
    NaturalLanguage__ChineseSimplified,
    #[strum(serialize = "Natural Language :: Chinese (Traditional)")]
    NaturalLanguage__ChineseTraditional,
    #[strum(serialize = "Natural Language :: Croatian")]
    NaturalLanguage__Croatian,
    #[strum(serialize = "Natural Language :: Czech")]
    NaturalLanguage__Czech,
    #[strum(serialize = "Natural Language :: Danish")]
    NaturalLanguage__Danish,
    #[strum(serialize = "Natural Language :: Dutch")]
    NaturalLanguage__Dutch,
    #[strum(serialize = "Natural Language :: English")]
    NaturalLanguage__English,
    #[strum(serialize = "Natural Language :: Esperanto")]
    NaturalLanguage__Esperanto,
    #[strum(serialize = "Natural Language :: Finnish")]
    NaturalLanguage__Finnish,
    #[strum(serialize = "Natural Language :: French")]
    NaturalLanguage__French,
    #[strum(serialize = "Natural Language :: Galician")]
    NaturalLanguage__Galician,
    #[strum(serialize = "Natural Language :: Georgian")]
    NaturalLanguage__Georgian,
    #[strum(serialize = "Natural Language :: German")]
    NaturalLanguage__German,
    #[strum(serialize = "Natural Language :: Greek")]
    NaturalLanguage__Greek,
    #[strum(serialize = "Natural Language :: Hebrew")]
    NaturalLanguage__Hebrew,
    #[strum(serialize = "Natural Language :: Hindi")]
    NaturalLanguage__Hindi,
    #[strum(serialize = "Natural Language :: Hungarian")]
    NaturalLanguage__Hungarian,
    #[strum(serialize = "Natural Language :: Icelandic")]
    NaturalLanguage__Icelandic,
    #[strum(serialize = "Natural Language :: Indonesian")]
    NaturalLanguage__Indonesian,
    #[strum(serialize = "Natural Language :: Irish")]
    NaturalLanguage__Irish,
    #[strum(serialize = "Natural Language :: Italian")]
    NaturalLanguage__Italian,
    #[strum(serialize = "Natural Language :: Japanese")]
    NaturalLanguage__Japanese,
    #[strum(serialize = "Natural Language :: Javanese")]
    NaturalLanguage__Javanese,
    #[strum(serialize = "Natural Language :: Korean")]
    NaturalLanguage__Korean,
    #[strum(serialize = "Natural Language :: Latin")]
    NaturalLanguage__Latin,
    #[strum(serialize = "Natural Language :: Latvian")]
    NaturalLanguage__Latvian,
    #[strum(serialize = "Natural Language :: Lithuanian")]
    NaturalLanguage__Lithuanian,
    #[strum(serialize = "Natural Language :: Macedonian")]
    NaturalLanguage__Macedonian,
    #[strum(serialize = "Natural Language :: Malay")]
    NaturalLanguage__Malay,
    #[strum(serialize = "Natural Language :: Marathi")]
    NaturalLanguage__Marathi,
    #[strum(serialize = "Natural Language :: Nepali")]
    NaturalLanguage__Nepali,
    #[strum(serialize = "Natural Language :: Norwegian")]
    NaturalLanguage__Norwegian,
    #[strum(serialize = "Natural Language :: Panjabi")]
    NaturalLanguage__Panjabi,
    #[strum(serialize = "Natural Language :: Persian")]
    NaturalLanguage__Persian,
    #[strum(serialize = "Natural Language :: Polish")]
    NaturalLanguage__Polish,
    #[strum(serialize = "Natural Language :: Portuguese")]
    NaturalLanguage__Portuguese,
    #[strum(serialize = "Natural Language :: Portuguese (Brazilian)")]
    NaturalLanguage__PortugueseBrazilian,
    #[strum(serialize = "Natural Language :: Romanian")]
    NaturalLanguage__Romanian,
    #[strum(serialize = "Natural Language :: Russian")]
    NaturalLanguage__Russian,
    #[strum(serialize = "Natural Language :: Serbian")]
    NaturalLanguage__Serbian,
    #[strum(serialize = "Natural Language :: Slovak")]
    NaturalLanguage__Slovak,
    #[strum(serialize = "Natural Language :: Slovenian")]
    NaturalLanguage__Slovenian,
    #[strum(serialize = "Natural Language :: Spanish")]
    NaturalLanguage__Spanish,
    #[strum(serialize = "Natural Language :: Swedish")]
    NaturalLanguage__Swedish,
    #[strum(serialize = "Natural Language :: Tamil")]
    NaturalLanguage__Tamil,
    #[strum(serialize = "Natural Language :: Telugu")]
    NaturalLanguage__Telugu,
    #[strum(serialize = "Natural Language :: Thai")]
    NaturalLanguage__Thai,
    #[strum(serialize = "Natural Language :: Tibetan")]
    NaturalLanguage__Tibetan,
    #[strum(serialize = "Natural Language :: Turkish")]
    NaturalLanguage__Turkish,
    #[strum(serialize = "Natural Language :: Ukrainian")]
    NaturalLanguage__Ukrainian,
    #[strum(serialize = "Natural Language :: Urdu")]
    NaturalLanguage__Urdu,
    #[strum(serialize = "Natural Language :: Vietnamese")]
    NaturalLanguage__Vietnamese,
    #[strum(serialize = "Operating System :: Android")]
    OperatingSystem__Android,
    #[strum(serialize = "Operating System :: BeOS")]
    OperatingSystem__BeOS,
    #[strum(serialize = "Operating System :: MacOS")]
    OperatingSystem__MacOS,
    #[strum(serialize = "Operating System :: MacOS :: MacOS 9")]
    OperatingSystem__MacOS__MacOS9,
    #[strum(serialize = "Operating System :: MacOS :: MacOS X")]
    OperatingSystem__MacOS__MacOSX,
    #[strum(serialize = "Operating System :: Microsoft")]
    OperatingSystem__Microsoft,
    #[strum(serialize = "Operating System :: Microsoft :: MS-DOS")]
    OperatingSystem__Microsoft__MSDOS,
    #[strum(serialize = "Operating System :: Microsoft :: Windows")]
    OperatingSystem__Microsoft__Windows,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows 3.1 or Earlier")]
    OperatingSystem__Microsoft__Windows__Windows3_1orEarlier,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows 7")]
    OperatingSystem__Microsoft__Windows__Windows7,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows 8")]
    OperatingSystem__Microsoft__Windows__Windows8,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows 8.1")]
    OperatingSystem__Microsoft__Windows__Windows8_1,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows 10")]
    OperatingSystem__Microsoft__Windows__Windows10,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows 11")]
    OperatingSystem__Microsoft__Windows__Windows11,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows 95/98/2000")]
    OperatingSystem__Microsoft__Windows__Windows95982000,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows CE")]
    OperatingSystem__Microsoft__Windows__WindowsCE,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows NT/2000")]
    OperatingSystem__Microsoft__Windows__WindowsNT2000,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows Server 2003")]
    OperatingSystem__Microsoft__Windows__WindowsServer2003,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows Server 2008")]
    OperatingSystem__Microsoft__Windows__WindowsServer2008,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows Vista")]
    OperatingSystem__Microsoft__Windows__WindowsVista,
    #[strum(serialize = "Operating System :: Microsoft :: Windows :: Windows XP")]
    OperatingSystem__Microsoft__Windows__WindowsXP,
    #[strum(serialize = "Operating System :: OS Independent")]
    OperatingSystem__OSIndependent,
    #[strum(serialize = "Operating System :: OS/2")]
    OperatingSystem__OS2,
    #[strum(serialize = "Operating System :: Other OS")]
    OperatingSystem__OtherOS,
    #[strum(serialize = "Operating System :: PDA Systems")]
    OperatingSystem__PDASystems,
    #[strum(serialize = "Operating System :: POSIX")]
    OperatingSystem__POSIX,
    #[strum(serialize = "Operating System :: POSIX :: AIX")]
    OperatingSystem__POSIX__AIX,
    #[strum(serialize = "Operating System :: POSIX :: BSD")]
    OperatingSystem__POSIX__BSD,
    #[strum(serialize = "Operating System :: POSIX :: BSD :: BSD/OS")]
    OperatingSystem__POSIX__BSD__BSDOS,
    #[strum(serialize = "Operating System :: POSIX :: BSD :: FreeBSD")]
    OperatingSystem__POSIX__BSD__FreeBSD,
    #[strum(serialize = "Operating System :: POSIX :: BSD :: NetBSD")]
    OperatingSystem__POSIX__BSD__NetBSD,
    #[strum(serialize = "Operating System :: POSIX :: BSD :: OpenBSD")]
    OperatingSystem__POSIX__BSD__OpenBSD,
    #[strum(serialize = "Operating System :: POSIX :: GNU Hurd")]
    OperatingSystem__POSIX__GNUHurd,
    #[strum(serialize = "Operating System :: POSIX :: HP-UX")]
    OperatingSystem__POSIX__HPUX,
    #[strum(serialize = "Operating System :: POSIX :: IRIX")]
    OperatingSystem__POSIX__IRIX,
    #[strum(serialize = "Operating System :: POSIX :: Linux")]
    OperatingSystem__POSIX__Linux,
    #[strum(serialize = "Operating System :: POSIX :: Other")]
    OperatingSystem__POSIX__Other,
    #[strum(serialize = "Operating System :: POSIX :: SCO")]
    OperatingSystem__POSIX__SCO,
    #[strum(serialize = "Operating System :: POSIX :: SunOS/Solaris")]
    OperatingSystem__POSIX__SunOSSolaris,
    #[strum(serialize = "Operating System :: PalmOS")]
    OperatingSystem__PalmOS,
    #[strum(serialize = "Operating System :: RISC OS")]
    OperatingSystem__RISCOS,
    #[strum(serialize = "Operating System :: Unix")]
    OperatingSystem__Unix,
    #[strum(serialize = "Operating System :: iOS")]
    OperatingSystem__iOS,
    #[strum(serialize = "Programming Language :: APL")]
    ProgrammingLanguage__APL,
    #[strum(serialize = "Programming Language :: ASP")]
    ProgrammingLanguage__ASP,
    #[strum(serialize = "Programming Language :: Ada")]
    ProgrammingLanguage__Ada,
    #[strum(serialize = "Programming Language :: Assembly")]
    ProgrammingLanguage__Assembly,
    #[strum(serialize = "Programming Language :: Awk")]
    ProgrammingLanguage__Awk,
    #[strum(serialize = "Programming Language :: Basic")]
    ProgrammingLanguage__Basic,
    #[strum(serialize = "Programming Language :: C")]
    ProgrammingLanguage__C,
    #[strum(serialize = "Programming Language :: C#")]
    ProgrammingLanguage__Csharp,
    #[strum(serialize = "Programming Language :: C++")]
    ProgrammingLanguage__Cplusplus,
    #[strum(serialize = "Programming Language :: Cold Fusion")]
    ProgrammingLanguage__ColdFusion,
    #[strum(serialize = "Programming Language :: Cython")]
    ProgrammingLanguage__Cython,
    #[strum(serialize = "Programming Language :: D")]
    ProgrammingLanguage__D,
    #[strum(serialize = "Programming Language :: Delphi/Kylix")]
    ProgrammingLanguage__DelphiKylix,
    #[strum(serialize = "Programming Language :: Dylan")]
    ProgrammingLanguage__Dylan,
    #[strum(serialize = "Programming Language :: Eiffel")]
    ProgrammingLanguage__Eiffel,
    #[strum(serialize = "Programming Language :: Emacs-Lisp")]
    ProgrammingLanguage__EmacsLisp,
    #[strum(serialize = "Programming Language :: Erlang")]
    ProgrammingLanguage__Erlang,
    #[strum(serialize = "Programming Language :: Euler")]
    ProgrammingLanguage__Euler,
    #[strum(serialize = "Programming Language :: Euphoria")]
    ProgrammingLanguage__Euphoria,
    #[strum(serialize = "Programming Language :: F#")]
    ProgrammingLanguage__Fsharp,
    #[strum(serialize = "Programming Language :: Forth")]
    ProgrammingLanguage__Forth,
    #[strum(serialize = "Programming Language :: Fortran")]
    ProgrammingLanguage__Fortran,
    #[strum(serialize = "Programming Language :: Go")]
    ProgrammingLanguage__Go,
    #[strum(serialize = "Programming Language :: Haskell")]
    ProgrammingLanguage__Haskell,
    #[strum(serialize = "Programming Language :: Hy")]
    ProgrammingLanguage__Hy,
    #[strum(serialize = "Programming Language :: Java")]
    ProgrammingLanguage__Java,
    #[strum(serialize = "Programming Language :: JavaScript")]
    ProgrammingLanguage__JavaScript,
    #[strum(serialize = "Programming Language :: Kotlin")]
    ProgrammingLanguage__Kotlin,
    #[strum(serialize = "Programming Language :: Lisp")]
    ProgrammingLanguage__Lisp,
    #[strum(serialize = "Programming Language :: Logo")]
    ProgrammingLanguage__Logo,
    #[strum(serialize = "Programming Language :: Lua")]
    ProgrammingLanguage__Lua,
    #[strum(serialize = "Programming Language :: ML")]
    ProgrammingLanguage__ML,
    #[strum(serialize = "Programming Language :: Modula")]
    ProgrammingLanguage__Modula,
    #[strum(serialize = "Programming Language :: OCaml")]
    ProgrammingLanguage__OCaml,
    #[strum(serialize = "Programming Language :: Object Pascal")]
    ProgrammingLanguage__ObjectPascal,
    #[strum(serialize = "Programming Language :: Objective C")]
    ProgrammingLanguage__ObjectiveC,
    #[strum(serialize = "Programming Language :: Other")]
    ProgrammingLanguage__Other,
    #[strum(serialize = "Programming Language :: Other Scripting Engines")]
    ProgrammingLanguage__OtherScriptingEngines,
    #[strum(serialize = "Programming Language :: PHP")]
    ProgrammingLanguage__PHP,
    #[strum(serialize = "Programming Language :: PL/SQL")]
    ProgrammingLanguage__PLSQL,
    #[strum(serialize = "Programming Language :: PROGRESS")]
    ProgrammingLanguage__PROGRESS,
    #[strum(serialize = "Programming Language :: Pascal")]
    ProgrammingLanguage__Pascal,
    #[strum(serialize = "Programming Language :: Perl")]
    ProgrammingLanguage__Perl,
    #[strum(serialize = "Programming Language :: Pike")]
    ProgrammingLanguage__Pike,
    #[strum(serialize = "Programming Language :: Pliant")]
    ProgrammingLanguage__Pliant,
    #[strum(serialize = "Programming Language :: Prolog")]
    ProgrammingLanguage__Prolog,
    #[strum(serialize = "Programming Language :: Python")]
    ProgrammingLanguage__Python,
    #[strum(serialize = "Programming Language :: Python :: 2")]
    ProgrammingLanguage__Python__2,
    #[strum(serialize = "Programming Language :: Python :: 2 :: Only")]
    ProgrammingLanguage__Python__2__Only,
    #[strum(serialize = "Programming Language :: Python :: 2.3")]
    ProgrammingLanguage__Python__2_3,
    #[strum(serialize = "Programming Language :: Python :: 2.4")]
    ProgrammingLanguage__Python__2_4,
    #[strum(serialize = "Programming Language :: Python :: 2.5")]
    ProgrammingLanguage__Python__2_5,
    #[strum(serialize = "Programming Language :: Python :: 2.6")]
    ProgrammingLanguage__Python__2_6,
    #[strum(serialize = "Programming Language :: Python :: 2.7")]
    ProgrammingLanguage__Python__2_7,
    #[strum(serialize = "Programming Language :: Python :: 3")]
    ProgrammingLanguage__Python__3,
    #[strum(serialize = "Programming Language :: Python :: 3 :: Only")]
    ProgrammingLanguage__Python__3__Only,
    #[strum(serialize = "Programming Language :: Python :: 3.0")]
    ProgrammingLanguage__Python__3_0,
    #[strum(serialize = "Programming Language :: Python :: 3.1")]
    ProgrammingLanguage__Python__3_1,
    #[strum(serialize = "Programming Language :: Python :: 3.2")]
    ProgrammingLanguage__Python__3_2,
    #[strum(serialize = "Programming Language :: Python :: 3.3")]
    ProgrammingLanguage__Python__3_3,
    #[strum(serialize = "Programming Language :: Python :: 3.4")]
    ProgrammingLanguage__Python__3_4,
    #[strum(serialize = "Programming Language :: Python :: 3.5")]
    ProgrammingLanguage__Python__3_5,
    #[strum(serialize = "Programming Language :: Python :: 3.6")]
    ProgrammingLanguage__Python__3_6,
    #[strum(serialize = "Programming Language :: Python :: 3.7")]
    ProgrammingLanguage__Python__3_7,
    #[strum(serialize = "Programming Language :: Python :: 3.8")]
    ProgrammingLanguage__Python__3_8,
    #[strum(serialize = "Programming Language :: Python :: 3.9")]
    ProgrammingLanguage__Python__3_9,
    #[strum(serialize = "Programming Language :: Python :: 3.10")]
    ProgrammingLanguage__Python__3_10,
    #[strum(serialize = "Programming Language :: Python :: 3.11")]
    ProgrammingLanguage__Python__3_11,
    #[strum(serialize = "Programming Language :: Python :: 3.12")]
    ProgrammingLanguage__Python__3_12,
    #[strum(serialize = "Programming Language :: Python :: 3.13")]
    ProgrammingLanguage__Python__3_13,
    #[strum(serialize = "Programming Language :: Python :: 3.14")]
    ProgrammingLanguage__Python__3_14,
    #[strum(serialize = "Programming Language :: Python :: Implementation")]
    ProgrammingLanguage__Python__Implementation,
    #[strum(serialize = "Programming Language :: Python :: Implementation :: CPython")]
    ProgrammingLanguage__Python__Implementation__CPython,
    #[strum(serialize = "Programming Language :: Python :: Implementation :: IronPython")]
    ProgrammingLanguage__Python__Implementation__IronPython,
    #[strum(serialize = "Programming Language :: Python :: Implementation :: Jython")]
    ProgrammingLanguage__Python__Implementation__Jython,
    #[strum(serialize = "Programming Language :: Python :: Implementation :: MicroPython")]
    ProgrammingLanguage__Python__Implementation__MicroPython,
    #[strum(serialize = "Programming Language :: Python :: Implementation :: PyPy")]
    ProgrammingLanguage__Python__Implementation__PyPy,
    #[strum(serialize = "Programming Language :: Python :: Implementation :: Stackless")]
    ProgrammingLanguage__Python__Implementation__Stackless,
    #[strum(serialize = "Programming Language :: R")]
    ProgrammingLanguage__R,
    #[strum(serialize = "Programming Language :: REBOL")]
    ProgrammingLanguage__REBOL,
    #[strum(serialize = "Programming Language :: Rexx")]
    ProgrammingLanguage__Rexx,
    #[strum(serialize = "Programming Language :: Ruby")]
    ProgrammingLanguage__Ruby,
    #[strum(serialize = "Programming Language :: Rust")]
    ProgrammingLanguage__Rust,
    #[strum(serialize = "Programming Language :: SQL")]
    ProgrammingLanguage__SQL,
    #[strum(serialize = "Programming Language :: Scheme")]
    ProgrammingLanguage__Scheme,
    #[strum(serialize = "Programming Language :: Simula")]
    ProgrammingLanguage__Simula,
    #[strum(serialize = "Programming Language :: Smalltalk")]
    ProgrammingLanguage__Smalltalk,
    #[strum(serialize = "Programming Language :: Tcl")]
    ProgrammingLanguage__Tcl,
    #[strum(serialize = "Programming Language :: Unix Shell")]
    ProgrammingLanguage__UnixShell,
    #[strum(serialize = "Programming Language :: Visual Basic")]
    ProgrammingLanguage__VisualBasic,
    #[strum(serialize = "Programming Language :: XBasic")]
    ProgrammingLanguage__XBasic,
    #[strum(serialize = "Programming Language :: YACC")]
    ProgrammingLanguage__YACC,
    #[strum(serialize = "Programming Language :: Zope")]
    ProgrammingLanguage__Zope,
    #[strum(serialize = "Topic :: Adaptive Technologies")]
    Topic__AdaptiveTechnologies,
    #[strum(serialize = "Topic :: Artistic Software")]
    Topic__ArtisticSoftware,
    #[strum(serialize = "Topic :: Communications")]
    Topic__Communications,
    #[strum(serialize = "Topic :: Communications :: BBS")]
    Topic__Communications__BBS,
    #[strum(serialize = "Topic :: Communications :: Chat")]
    Topic__Communications__Chat,
    #[strum(serialize = "Topic :: Communications :: Chat :: ICQ")]
    Topic__Communications__Chat__ICQ,
    #[strum(serialize = "Topic :: Communications :: Chat :: Internet Relay Chat")]
    Topic__Communications__Chat__InternetRelayChat,
    #[strum(serialize = "Topic :: Communications :: Chat :: Unix Talk")]
    Topic__Communications__Chat__UnixTalk,
    #[strum(serialize = "Topic :: Communications :: Conferencing")]
    Topic__Communications__Conferencing,
    #[strum(serialize = "Topic :: Communications :: Email")]
    Topic__Communications__Email,
    #[strum(serialize = "Topic :: Communications :: Email :: Address Book")]
    Topic__Communications__Email__AddressBook,
    #[strum(serialize = "Topic :: Communications :: Email :: Email Clients (MUA)")]
    Topic__Communications__Email__EmailClientsMUA,
    #[strum(serialize = "Topic :: Communications :: Email :: Filters")]
    Topic__Communications__Email__Filters,
    #[strum(serialize = "Topic :: Communications :: Email :: Mail Transport Agents")]
    Topic__Communications__Email__MailTransportAgents,
    #[strum(serialize = "Topic :: Communications :: Email :: Mailing List Servers")]
    Topic__Communications__Email__MailingListServers,
    #[strum(serialize = "Topic :: Communications :: Email :: Post-Office")]
    Topic__Communications__Email__PostOffice,
    #[strum(serialize = "Topic :: Communications :: Email :: Post-Office :: IMAP")]
    Topic__Communications__Email__PostOffice__IMAP,
    #[strum(serialize = "Topic :: Communications :: Email :: Post-Office :: POP3")]
    Topic__Communications__Email__PostOffice__POP3,
    #[strum(serialize = "Topic :: Communications :: FIDO")]
    Topic__Communications__FIDO,
    #[strum(serialize = "Topic :: Communications :: Fax")]
    Topic__Communications__Fax,
    #[strum(serialize = "Topic :: Communications :: File Sharing")]
    Topic__Communications__FileSharing,
    #[strum(serialize = "Topic :: Communications :: File Sharing :: Gnutella")]
    Topic__Communications__FileSharing__Gnutella,
    #[strum(serialize = "Topic :: Communications :: File Sharing :: Napster")]
    Topic__Communications__FileSharing__Napster,
    #[strum(serialize = "Topic :: Communications :: Ham Radio")]
    Topic__Communications__HamRadio,
    #[strum(serialize = "Topic :: Communications :: Internet Phone")]
    Topic__Communications__InternetPhone,
    #[strum(serialize = "Topic :: Communications :: Telephony")]
    Topic__Communications__Telephony,
    #[strum(serialize = "Topic :: Communications :: Usenet News")]
    Topic__Communications__UsenetNews,
    #[strum(serialize = "Topic :: Database")]
    Topic__Database,
    #[strum(serialize = "Topic :: Database :: Database Engines/Servers")]
    Topic__Database__DatabaseEnginesServers,
    #[strum(serialize = "Topic :: Database :: Front-Ends")]
    Topic__Database__FrontEnds,
    #[strum(serialize = "Topic :: Desktop Environment")]
    Topic__DesktopEnvironment,
    #[strum(serialize = "Topic :: Desktop Environment :: File Managers")]
    Topic__DesktopEnvironment__FileManagers,
    #[strum(serialize = "Topic :: Desktop Environment :: GNUstep")]
    Topic__DesktopEnvironment__GNUstep,
    #[strum(serialize = "Topic :: Desktop Environment :: Gnome")]
    Topic__DesktopEnvironment__Gnome,
    #[strum(serialize = "Topic :: Desktop Environment :: K Desktop Environment (KDE)")]
    Topic__DesktopEnvironment__KDesktopEnvironmentKDE,
    #[strum(serialize = "Topic :: Desktop Environment :: K Desktop Environment (KDE) :: Themes")]
    Topic__DesktopEnvironment__KDesktopEnvironmentKDE__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: PicoGUI")]
    Topic__DesktopEnvironment__PicoGUI,
    #[strum(serialize = "Topic :: Desktop Environment :: PicoGUI :: Applications")]
    Topic__DesktopEnvironment__PicoGUI__Applications,
    #[strum(serialize = "Topic :: Desktop Environment :: PicoGUI :: Themes")]
    Topic__DesktopEnvironment__PicoGUI__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Screen Savers")]
    Topic__DesktopEnvironment__ScreenSavers,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers")]
    Topic__DesktopEnvironment__WindowManagers,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Afterstep")]
    Topic__DesktopEnvironment__WindowManagers__Afterstep,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Afterstep :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__Afterstep__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Applets")]
    Topic__DesktopEnvironment__WindowManagers__Applets,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Blackbox")]
    Topic__DesktopEnvironment__WindowManagers__Blackbox,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Blackbox :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__Blackbox__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: CTWM")]
    Topic__DesktopEnvironment__WindowManagers__CTWM,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: CTWM :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__CTWM__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Enlightenment")]
    Topic__DesktopEnvironment__WindowManagers__Enlightenment,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Enlightenment :: Epplets"
    )]
    Topic__DesktopEnvironment__WindowManagers__Enlightenment__Epplets,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Enlightenment :: Themes DR15"
    )]
    Topic__DesktopEnvironment__WindowManagers__Enlightenment__ThemesDR15,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Enlightenment :: Themes DR16"
    )]
    Topic__DesktopEnvironment__WindowManagers__Enlightenment__ThemesDR16,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Enlightenment :: Themes DR17"
    )]
    Topic__DesktopEnvironment__WindowManagers__Enlightenment__ThemesDR17,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: FVWM")]
    Topic__DesktopEnvironment__WindowManagers__FVWM,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: FVWM :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__FVWM__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Fluxbox")]
    Topic__DesktopEnvironment__WindowManagers__Fluxbox,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Fluxbox :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__Fluxbox__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: IceWM")]
    Topic__DesktopEnvironment__WindowManagers__IceWM,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: IceWM :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__IceWM__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: MetaCity")]
    Topic__DesktopEnvironment__WindowManagers__MetaCity,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: MetaCity :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__MetaCity__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Oroborus")]
    Topic__DesktopEnvironment__WindowManagers__Oroborus,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Oroborus :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__Oroborus__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Sawfish")]
    Topic__DesktopEnvironment__WindowManagers__Sawfish,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Sawfish :: Themes 0.30"
    )]
    Topic__DesktopEnvironment__WindowManagers__Sawfish__Themes0_30,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Sawfish :: Themes pre-0.30"
    )]
    Topic__DesktopEnvironment__WindowManagers__Sawfish__Themespre0_30,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Waimea")]
    Topic__DesktopEnvironment__WindowManagers__Waimea,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Waimea :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__Waimea__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: Window Maker")]
    Topic__DesktopEnvironment__WindowManagers__WindowMaker,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Window Maker :: Applets"
    )]
    Topic__DesktopEnvironment__WindowManagers__WindowMaker__Applets,
    #[strum(
        serialize = "Topic :: Desktop Environment :: Window Managers :: Window Maker :: Themes"
    )]
    Topic__DesktopEnvironment__WindowManagers__WindowMaker__Themes,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: XFCE")]
    Topic__DesktopEnvironment__WindowManagers__XFCE,
    #[strum(serialize = "Topic :: Desktop Environment :: Window Managers :: XFCE :: Themes")]
    Topic__DesktopEnvironment__WindowManagers__XFCE__Themes,
    #[strum(serialize = "Topic :: Documentation")]
    Topic__Documentation,
    #[strum(serialize = "Topic :: Documentation :: Sphinx")]
    Topic__Documentation__Sphinx,
    #[strum(serialize = "Topic :: Education")]
    Topic__Education,
    #[strum(serialize = "Topic :: Education :: Computer Aided Instruction (CAI)")]
    Topic__Education__ComputerAidedInstructionCAI,
    #[strum(serialize = "Topic :: Education :: Testing")]
    Topic__Education__Testing,
    #[strum(serialize = "Topic :: File Formats")]
    Topic__FileFormats,
    #[strum(serialize = "Topic :: File Formats :: JSON")]
    Topic__FileFormats__JSON,
    #[strum(serialize = "Topic :: File Formats :: JSON :: JSON Schema")]
    Topic__FileFormats__JSON__JSONSchema,
    #[strum(serialize = "Topic :: Games/Entertainment")]
    Topic__GamesEntertainment,
    #[strum(serialize = "Topic :: Games/Entertainment :: Arcade")]
    Topic__GamesEntertainment__Arcade,
    #[strum(serialize = "Topic :: Games/Entertainment :: Board Games")]
    Topic__GamesEntertainment__BoardGames,
    #[strum(serialize = "Topic :: Games/Entertainment :: First Person Shooters")]
    Topic__GamesEntertainment__FirstPersonShooters,
    #[strum(serialize = "Topic :: Games/Entertainment :: Fortune Cookies")]
    Topic__GamesEntertainment__FortuneCookies,
    #[strum(serialize = "Topic :: Games/Entertainment :: Multi-User Dungeons (MUD)")]
    Topic__GamesEntertainment__MultiUserDungeonsMUD,
    #[strum(serialize = "Topic :: Games/Entertainment :: Puzzle Games")]
    Topic__GamesEntertainment__PuzzleGames,
    #[strum(serialize = "Topic :: Games/Entertainment :: Real Time Strategy")]
    Topic__GamesEntertainment__RealTimeStrategy,
    #[strum(serialize = "Topic :: Games/Entertainment :: Role-Playing")]
    Topic__GamesEntertainment__RolePlaying,
    #[strum(serialize = "Topic :: Games/Entertainment :: Side-Scrolling/Arcade Games")]
    Topic__GamesEntertainment__SideScrollingArcadeGames,
    #[strum(serialize = "Topic :: Games/Entertainment :: Simulation")]
    Topic__GamesEntertainment__Simulation,
    #[strum(serialize = "Topic :: Games/Entertainment :: Turn Based Strategy")]
    Topic__GamesEntertainment__TurnBasedStrategy,
    #[strum(serialize = "Topic :: Home Automation")]
    Topic__HomeAutomation,
    #[strum(serialize = "Topic :: Internet")]
    Topic__Internet,
    #[strum(serialize = "Topic :: Internet :: File Transfer Protocol (FTP)")]
    Topic__Internet__FileTransferProtocolFTP,
    #[strum(serialize = "Topic :: Internet :: Finger")]
    Topic__Internet__Finger,
    #[strum(serialize = "Topic :: Internet :: Log Analysis")]
    Topic__Internet__LogAnalysis,
    #[strum(serialize = "Topic :: Internet :: Name Service (DNS)")]
    Topic__Internet__NameServiceDNS,
    #[strum(serialize = "Topic :: Internet :: Proxy Servers")]
    Topic__Internet__ProxyServers,
    #[strum(serialize = "Topic :: Internet :: WAP")]
    Topic__Internet__WAP,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP")]
    Topic__Internet__WWWHTTP,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Browsers")]
    Topic__Internet__WWWHTTP__Browsers,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Dynamic Content")]
    Topic__Internet__WWWHTTP__DynamicContent,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Dynamic Content :: CGI Tools/Libraries")]
    Topic__Internet__WWWHTTP__DynamicContent__CGIToolsLibraries,
    #[strum(
        serialize = "Topic :: Internet :: WWW/HTTP :: Dynamic Content :: Content Management System"
    )]
    Topic__Internet__WWWHTTP__DynamicContent__ContentManagementSystem,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Dynamic Content :: Message Boards")]
    Topic__Internet__WWWHTTP__DynamicContent__MessageBoards,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Dynamic Content :: News/Diary")]
    Topic__Internet__WWWHTTP__DynamicContent__NewsDiary,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Dynamic Content :: Page Counters")]
    Topic__Internet__WWWHTTP__DynamicContent__PageCounters,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Dynamic Content :: Wiki")]
    Topic__Internet__WWWHTTP__DynamicContent__Wiki,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: HTTP Servers")]
    Topic__Internet__WWWHTTP__HTTPServers,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Indexing/Search")]
    Topic__Internet__WWWHTTP__IndexingSearch,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Session")]
    Topic__Internet__WWWHTTP__Session,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Site Management")]
    Topic__Internet__WWWHTTP__SiteManagement,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: Site Management :: Link Checking")]
    Topic__Internet__WWWHTTP__SiteManagement__LinkChecking,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: WSGI")]
    Topic__Internet__WWWHTTP__WSGI,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: WSGI :: Application")]
    Topic__Internet__WWWHTTP__WSGI__Application,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: WSGI :: Middleware")]
    Topic__Internet__WWWHTTP__WSGI__Middleware,
    #[strum(serialize = "Topic :: Internet :: WWW/HTTP :: WSGI :: Server")]
    Topic__Internet__WWWHTTP__WSGI__Server,
    #[strum(serialize = "Topic :: Internet :: XMPP")]
    Topic__Internet__XMPP,
    #[strum(serialize = "Topic :: Internet :: Z39.50")]
    Topic__Internet__Z39_50,
    #[strum(serialize = "Topic :: Multimedia")]
    Topic__Multimedia,
    #[strum(serialize = "Topic :: Multimedia :: Graphics")]
    Topic__Multimedia__Graphics,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: 3D Modeling")]
    Topic__Multimedia__Graphics__3DModeling,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: 3D Rendering")]
    Topic__Multimedia__Graphics__3DRendering,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Capture")]
    Topic__Multimedia__Graphics__Capture,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Capture :: Digital Camera")]
    Topic__Multimedia__Graphics__Capture__DigitalCamera,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Capture :: Scanners")]
    Topic__Multimedia__Graphics__Capture__Scanners,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Capture :: Screen Capture")]
    Topic__Multimedia__Graphics__Capture__ScreenCapture,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Editors")]
    Topic__Multimedia__Graphics__Editors,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Editors :: Raster-Based")]
    Topic__Multimedia__Graphics__Editors__RasterBased,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Editors :: Vector-Based")]
    Topic__Multimedia__Graphics__Editors__VectorBased,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Graphics Conversion")]
    Topic__Multimedia__Graphics__GraphicsConversion,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Presentation")]
    Topic__Multimedia__Graphics__Presentation,
    #[strum(serialize = "Topic :: Multimedia :: Graphics :: Viewers")]
    Topic__Multimedia__Graphics__Viewers,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio")]
    Topic__Multimedia__SoundAudio,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Analysis")]
    Topic__Multimedia__SoundAudio__Analysis,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: CD Audio")]
    Topic__Multimedia__SoundAudio__CDAudio,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: CD Audio :: CD Playing")]
    Topic__Multimedia__SoundAudio__CDAudio__CDPlaying,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: CD Audio :: CD Ripping")]
    Topic__Multimedia__SoundAudio__CDAudio__CDRipping,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: CD Audio :: CD Writing")]
    Topic__Multimedia__SoundAudio__CDAudio__CDWriting,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Capture/Recording")]
    Topic__Multimedia__SoundAudio__CaptureRecording,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Conversion")]
    Topic__Multimedia__SoundAudio__Conversion,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Editors")]
    Topic__Multimedia__SoundAudio__Editors,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: MIDI")]
    Topic__Multimedia__SoundAudio__MIDI,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Mixers")]
    Topic__Multimedia__SoundAudio__Mixers,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Players")]
    Topic__Multimedia__SoundAudio__Players,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Players :: MP3")]
    Topic__Multimedia__SoundAudio__Players__MP3,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Sound Synthesis")]
    Topic__Multimedia__SoundAudio__SoundSynthesis,
    #[strum(serialize = "Topic :: Multimedia :: Sound/Audio :: Speech")]
    Topic__Multimedia__SoundAudio__Speech,
    #[strum(serialize = "Topic :: Multimedia :: Video")]
    Topic__Multimedia__Video,
    #[strum(serialize = "Topic :: Multimedia :: Video :: Capture")]
    Topic__Multimedia__Video__Capture,
    #[strum(serialize = "Topic :: Multimedia :: Video :: Conversion")]
    Topic__Multimedia__Video__Conversion,
    #[strum(serialize = "Topic :: Multimedia :: Video :: Display")]
    Topic__Multimedia__Video__Display,
    #[strum(serialize = "Topic :: Multimedia :: Video :: Non-Linear Editor")]
    Topic__Multimedia__Video__NonLinearEditor,
    #[strum(serialize = "Topic :: Office/Business")]
    Topic__OfficeBusiness,
    #[strum(serialize = "Topic :: Office/Business :: Financial")]
    Topic__OfficeBusiness__Financial,
    #[strum(serialize = "Topic :: Office/Business :: Financial :: Accounting")]
    Topic__OfficeBusiness__Financial__Accounting,
    #[strum(serialize = "Topic :: Office/Business :: Financial :: Investment")]
    Topic__OfficeBusiness__Financial__Investment,
    #[strum(serialize = "Topic :: Office/Business :: Financial :: Point-Of-Sale")]
    Topic__OfficeBusiness__Financial__PointOfSale,
    #[strum(serialize = "Topic :: Office/Business :: Financial :: Spreadsheet")]
    Topic__OfficeBusiness__Financial__Spreadsheet,
    #[strum(serialize = "Topic :: Office/Business :: Groupware")]
    Topic__OfficeBusiness__Groupware,
    #[strum(serialize = "Topic :: Office/Business :: News/Diary")]
    Topic__OfficeBusiness__NewsDiary,
    #[strum(serialize = "Topic :: Office/Business :: Office Suites")]
    Topic__OfficeBusiness__OfficeSuites,
    #[strum(serialize = "Topic :: Office/Business :: Scheduling")]
    Topic__OfficeBusiness__Scheduling,
    #[strum(serialize = "Topic :: Other/Nonlisted Topic")]
    Topic__OtherNonlistedTopic,
    #[strum(serialize = "Topic :: Printing")]
    Topic__Printing,
    #[strum(serialize = "Topic :: Religion")]
    Topic__Religion,
    #[strum(serialize = "Topic :: Scientific/Engineering")]
    Topic__ScientificEngineering,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Artificial Intelligence")]
    Topic__ScientificEngineering__ArtificialIntelligence,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Artificial Life")]
    Topic__ScientificEngineering__ArtificialLife,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Astronomy")]
    Topic__ScientificEngineering__Astronomy,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Atmospheric Science")]
    Topic__ScientificEngineering__AtmosphericScience,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Bio-Informatics")]
    Topic__ScientificEngineering__BioInformatics,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Chemistry")]
    Topic__ScientificEngineering__Chemistry,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Electronic Design Automation (EDA)")]
    Topic__ScientificEngineering__ElectronicDesignAutomationEDA,
    #[strum(serialize = "Topic :: Scientific/Engineering :: GIS")]
    Topic__ScientificEngineering__GIS,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Human Machine Interfaces")]
    Topic__ScientificEngineering__HumanMachineInterfaces,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Hydrology")]
    Topic__ScientificEngineering__Hydrology,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Image Processing")]
    Topic__ScientificEngineering__ImageProcessing,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Image Recognition")]
    Topic__ScientificEngineering__ImageRecognition,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Information Analysis")]
    Topic__ScientificEngineering__InformationAnalysis,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Interface Engine/Protocol Translator")]
    Topic__ScientificEngineering__InterfaceEngineProtocolTranslator,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Mathematics")]
    Topic__ScientificEngineering__Mathematics,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Medical Science Apps.")]
    Topic__ScientificEngineering__MedicalScienceApps_,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Oceanography")]
    Topic__ScientificEngineering__Oceanography,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Physics")]
    Topic__ScientificEngineering__Physics,
    #[strum(serialize = "Topic :: Scientific/Engineering :: Visualization")]
    Topic__ScientificEngineering__Visualization,
    #[strum(serialize = "Topic :: Security")]
    Topic__Security,
    #[strum(serialize = "Topic :: Security :: Cryptography")]
    Topic__Security__Cryptography,
    #[strum(serialize = "Topic :: Sociology")]
    Topic__Sociology,
    #[strum(serialize = "Topic :: Sociology :: Genealogy")]
    Topic__Sociology__Genealogy,
    #[strum(serialize = "Topic :: Sociology :: History")]
    Topic__Sociology__History,
    #[strum(serialize = "Topic :: Software Development")]
    Topic__SoftwareDevelopment,
    #[strum(serialize = "Topic :: Software Development :: Assemblers")]
    Topic__SoftwareDevelopment__Assemblers,
    #[strum(serialize = "Topic :: Software Development :: Bug Tracking")]
    Topic__SoftwareDevelopment__BugTracking,
    #[strum(serialize = "Topic :: Software Development :: Build Tools")]
    Topic__SoftwareDevelopment__BuildTools,
    #[strum(serialize = "Topic :: Software Development :: Code Generators")]
    Topic__SoftwareDevelopment__CodeGenerators,
    #[strum(serialize = "Topic :: Software Development :: Compilers")]
    Topic__SoftwareDevelopment__Compilers,
    #[strum(serialize = "Topic :: Software Development :: Debuggers")]
    Topic__SoftwareDevelopment__Debuggers,
    #[strum(serialize = "Topic :: Software Development :: Disassemblers")]
    Topic__SoftwareDevelopment__Disassemblers,
    #[strum(serialize = "Topic :: Software Development :: Documentation")]
    Topic__SoftwareDevelopment__Documentation,
    #[strum(serialize = "Topic :: Software Development :: Embedded Systems")]
    Topic__SoftwareDevelopment__EmbeddedSystems,
    #[strum(
        serialize = "Topic :: Software Development :: Embedded Systems :: Controller Area Network (CAN)"
    )]
    Topic__SoftwareDevelopment__EmbeddedSystems__ControllerAreaNetworkCAN,
    #[strum(
        serialize = "Topic :: Software Development :: Embedded Systems :: Controller Area Network (CAN) :: CANopen"
    )]
    Topic__SoftwareDevelopment__EmbeddedSystems__ControllerAreaNetworkCAN__CANopen,
    #[strum(
        serialize = "Topic :: Software Development :: Embedded Systems :: Controller Area Network (CAN) :: J1939"
    )]
    Topic__SoftwareDevelopment__EmbeddedSystems__ControllerAreaNetworkCAN__J1939,
    #[strum(serialize = "Topic :: Software Development :: Internationalization")]
    Topic__SoftwareDevelopment__Internationalization,
    #[strum(serialize = "Topic :: Software Development :: Interpreters")]
    Topic__SoftwareDevelopment__Interpreters,
    #[strum(serialize = "Topic :: Software Development :: Libraries")]
    Topic__SoftwareDevelopment__Libraries,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: Application Frameworks")]
    Topic__SoftwareDevelopment__Libraries__ApplicationFrameworks,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: Java Libraries")]
    Topic__SoftwareDevelopment__Libraries__JavaLibraries,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: PHP Classes")]
    Topic__SoftwareDevelopment__Libraries__PHPClasses,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: Perl Modules")]
    Topic__SoftwareDevelopment__Libraries__PerlModules,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: Pike Modules")]
    Topic__SoftwareDevelopment__Libraries__PikeModules,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: Python Modules")]
    Topic__SoftwareDevelopment__Libraries__PythonModules,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: Ruby Modules")]
    Topic__SoftwareDevelopment__Libraries__RubyModules,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: Tcl Extensions")]
    Topic__SoftwareDevelopment__Libraries__TclExtensions,
    #[strum(serialize = "Topic :: Software Development :: Libraries :: pygame")]
    Topic__SoftwareDevelopment__Libraries__pygame,
    #[strum(serialize = "Topic :: Software Development :: Localization")]
    Topic__SoftwareDevelopment__Localization,
    #[strum(serialize = "Topic :: Software Development :: Object Brokering")]
    Topic__SoftwareDevelopment__ObjectBrokering,
    #[strum(serialize = "Topic :: Software Development :: Object Brokering :: CORBA")]
    Topic__SoftwareDevelopment__ObjectBrokering__CORBA,
    #[strum(serialize = "Topic :: Software Development :: Pre-processors")]
    Topic__SoftwareDevelopment__Preprocessors,
    #[strum(serialize = "Topic :: Software Development :: Quality Assurance")]
    Topic__SoftwareDevelopment__QualityAssurance,
    #[strum(serialize = "Topic :: Software Development :: Testing")]
    Topic__SoftwareDevelopment__Testing,
    #[strum(serialize = "Topic :: Software Development :: Testing :: Acceptance")]
    Topic__SoftwareDevelopment__Testing__Acceptance,
    #[strum(serialize = "Topic :: Software Development :: Testing :: BDD")]
    Topic__SoftwareDevelopment__Testing__BDD,
    #[strum(serialize = "Topic :: Software Development :: Testing :: Mocking")]
    Topic__SoftwareDevelopment__Testing__Mocking,
    #[strum(serialize = "Topic :: Software Development :: Testing :: Traffic Generation")]
    Topic__SoftwareDevelopment__Testing__TrafficGeneration,
    #[strum(serialize = "Topic :: Software Development :: Testing :: Unit")]
    Topic__SoftwareDevelopment__Testing__Unit,
    #[strum(serialize = "Topic :: Software Development :: User Interfaces")]
    Topic__SoftwareDevelopment__UserInterfaces,
    #[strum(serialize = "Topic :: Software Development :: Version Control")]
    Topic__SoftwareDevelopment__VersionControl,
    #[strum(serialize = "Topic :: Software Development :: Version Control :: Bazaar")]
    Topic__SoftwareDevelopment__VersionControl__Bazaar,
    #[strum(serialize = "Topic :: Software Development :: Version Control :: CVS")]
    Topic__SoftwareDevelopment__VersionControl__CVS,
    #[strum(serialize = "Topic :: Software Development :: Version Control :: Git")]
    Topic__SoftwareDevelopment__VersionControl__Git,
    #[strum(serialize = "Topic :: Software Development :: Version Control :: Mercurial")]
    Topic__SoftwareDevelopment__VersionControl__Mercurial,
    #[strum(serialize = "Topic :: Software Development :: Version Control :: RCS")]
    Topic__SoftwareDevelopment__VersionControl__RCS,
    #[strum(serialize = "Topic :: Software Development :: Version Control :: SCCS")]
    Topic__SoftwareDevelopment__VersionControl__SCCS,
    #[strum(serialize = "Topic :: Software Development :: Widget Sets")]
    Topic__SoftwareDevelopment__WidgetSets,
    #[strum(serialize = "Topic :: System")]
    Topic__System,
    #[strum(serialize = "Topic :: System :: Archiving")]
    Topic__System__Archiving,
    #[strum(serialize = "Topic :: System :: Archiving :: Backup")]
    Topic__System__Archiving__Backup,
    #[strum(serialize = "Topic :: System :: Archiving :: Compression")]
    Topic__System__Archiving__Compression,
    #[strum(serialize = "Topic :: System :: Archiving :: Mirroring")]
    Topic__System__Archiving__Mirroring,
    #[strum(serialize = "Topic :: System :: Archiving :: Packaging")]
    Topic__System__Archiving__Packaging,
    #[strum(serialize = "Topic :: System :: Benchmark")]
    Topic__System__Benchmark,
    #[strum(serialize = "Topic :: System :: Boot")]
    Topic__System__Boot,
    #[strum(serialize = "Topic :: System :: Boot :: Init")]
    Topic__System__Boot__Init,
    #[strum(serialize = "Topic :: System :: Clustering")]
    Topic__System__Clustering,
    #[strum(serialize = "Topic :: System :: Console Fonts")]
    Topic__System__ConsoleFonts,
    #[strum(serialize = "Topic :: System :: Distributed Computing")]
    Topic__System__DistributedComputing,
    #[strum(serialize = "Topic :: System :: Emulators")]
    Topic__System__Emulators,
    #[strum(serialize = "Topic :: System :: Filesystems")]
    Topic__System__Filesystems,
    #[strum(serialize = "Topic :: System :: Hardware")]
    Topic__System__Hardware,
    #[strum(serialize = "Topic :: System :: Hardware :: Hardware Drivers")]
    Topic__System__Hardware__HardwareDrivers,
    #[strum(serialize = "Topic :: System :: Hardware :: Mainframes")]
    Topic__System__Hardware__Mainframes,
    #[strum(serialize = "Topic :: System :: Hardware :: Symmetric Multi-processing")]
    Topic__System__Hardware__SymmetricMultiprocessing,
    #[strum(serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB)")]
    Topic__System__Hardware__UniversalSerialBusUSB,
    #[strum(serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Audio")]
    Topic__System__Hardware__UniversalSerialBusUSB__Audio,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Audio/Video (AV)"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__AudioVideoAV,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Communications Device Class (CDC)"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__CommunicationsDeviceClassCDC,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Diagnostic Device"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__DiagnosticDevice,
    #[strum(serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Hub")]
    Topic__System__Hardware__UniversalSerialBusUSB__Hub,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Human Interface Device (HID)"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__HumanInterfaceDeviceHID,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Mass Storage"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__MassStorage,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Miscellaneous"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__Miscellaneous,
    #[strum(serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Printer")]
    Topic__System__Hardware__UniversalSerialBusUSB__Printer,
    #[strum(serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Smart Card")]
    Topic__System__Hardware__UniversalSerialBusUSB__SmartCard,
    #[strum(serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Vendor")]
    Topic__System__Hardware__UniversalSerialBusUSB__Vendor,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Video (UVC)"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__VideoUVC,
    #[strum(
        serialize = "Topic :: System :: Hardware :: Universal Serial Bus (USB) :: Wireless Controller"
    )]
    Topic__System__Hardware__UniversalSerialBusUSB__WirelessController,
    #[strum(serialize = "Topic :: System :: Installation/Setup")]
    Topic__System__InstallationSetup,
    #[strum(serialize = "Topic :: System :: Logging")]
    Topic__System__Logging,
    #[strum(serialize = "Topic :: System :: Monitoring")]
    Topic__System__Monitoring,
    #[strum(serialize = "Topic :: System :: Networking")]
    Topic__System__Networking,
    #[strum(serialize = "Topic :: System :: Networking :: Firewalls")]
    Topic__System__Networking__Firewalls,
    #[strum(serialize = "Topic :: System :: Networking :: Monitoring")]
    Topic__System__Networking__Monitoring,
    #[strum(serialize = "Topic :: System :: Networking :: Monitoring :: Hardware Watchdog")]
    Topic__System__Networking__Monitoring__HardwareWatchdog,
    #[strum(serialize = "Topic :: System :: Networking :: Time Synchronization")]
    Topic__System__Networking__TimeSynchronization,
    #[strum(serialize = "Topic :: System :: Operating System")]
    Topic__System__OperatingSystem,
    #[strum(serialize = "Topic :: System :: Operating System Kernels")]
    Topic__System__OperatingSystemKernels,
    #[strum(serialize = "Topic :: System :: Operating System Kernels :: BSD")]
    Topic__System__OperatingSystemKernels__BSD,
    #[strum(serialize = "Topic :: System :: Operating System Kernels :: GNU Hurd")]
    Topic__System__OperatingSystemKernels__GNUHurd,
    #[strum(serialize = "Topic :: System :: Operating System Kernels :: Linux")]
    Topic__System__OperatingSystemKernels__Linux,
    #[strum(serialize = "Topic :: System :: Power (UPS)")]
    Topic__System__PowerUPS,
    #[strum(serialize = "Topic :: System :: Recovery Tools")]
    Topic__System__RecoveryTools,
    #[strum(serialize = "Topic :: System :: Shells")]
    Topic__System__Shells,
    #[strum(serialize = "Topic :: System :: Software Distribution")]
    Topic__System__SoftwareDistribution,
    #[strum(serialize = "Topic :: System :: System Shells")]
    Topic__System__SystemShells,
    #[strum(serialize = "Topic :: System :: Systems Administration")]
    Topic__System__SystemsAdministration,
    #[strum(serialize = "Topic :: System :: Systems Administration :: Authentication/Directory")]
    Topic__System__SystemsAdministration__AuthenticationDirectory,
    #[strum(
        serialize = "Topic :: System :: Systems Administration :: Authentication/Directory :: LDAP"
    )]
    Topic__System__SystemsAdministration__AuthenticationDirectory__LDAP,
    #[strum(
        serialize = "Topic :: System :: Systems Administration :: Authentication/Directory :: NIS"
    )]
    Topic__System__SystemsAdministration__AuthenticationDirectory__NIS,
    #[strum(serialize = "Topic :: Terminals")]
    Topic__Terminals,
    #[strum(serialize = "Topic :: Terminals :: Serial")]
    Topic__Terminals__Serial,
    #[strum(serialize = "Topic :: Terminals :: Telnet")]
    Topic__Terminals__Telnet,
    #[strum(serialize = "Topic :: Terminals :: Terminal Emulators/X Terminals")]
    Topic__Terminals__TerminalEmulatorsXTerminals,
    #[strum(serialize = "Topic :: Text Editors")]
    Topic__TextEditors,
    #[strum(serialize = "Topic :: Text Editors :: Documentation")]
    Topic__TextEditors__Documentation,
    #[strum(serialize = "Topic :: Text Editors :: Emacs")]
    Topic__TextEditors__Emacs,
    #[strum(serialize = "Topic :: Text Editors :: Integrated Development Environments (IDE)")]
    Topic__TextEditors__IntegratedDevelopmentEnvironmentsIDE,
    #[strum(serialize = "Topic :: Text Editors :: Text Processing")]
    Topic__TextEditors__TextProcessing,
    #[strum(serialize = "Topic :: Text Editors :: Word Processors")]
    Topic__TextEditors__WordProcessors,
    #[strum(serialize = "Topic :: Text Processing")]
    Topic__TextProcessing,
    #[strum(serialize = "Topic :: Text Processing :: Filters")]
    Topic__TextProcessing__Filters,
    #[strum(serialize = "Topic :: Text Processing :: Fonts")]
    Topic__TextProcessing__Fonts,
    #[strum(serialize = "Topic :: Text Processing :: General")]
    Topic__TextProcessing__General,
    #[strum(serialize = "Topic :: Text Processing :: Indexing")]
    Topic__TextProcessing__Indexing,
    #[strum(serialize = "Topic :: Text Processing :: Linguistic")]
    Topic__TextProcessing__Linguistic,
    #[strum(serialize = "Topic :: Text Processing :: Markup")]
    Topic__TextProcessing__Markup,
    #[strum(serialize = "Topic :: Text Processing :: Markup :: HTML")]
    Topic__TextProcessing__Markup__HTML,
    #[strum(serialize = "Topic :: Text Processing :: Markup :: LaTeX")]
    Topic__TextProcessing__Markup__LaTeX,
    #[strum(serialize = "Topic :: Text Processing :: Markup :: Markdown")]
    Topic__TextProcessing__Markup__Markdown,
    #[strum(serialize = "Topic :: Text Processing :: Markup :: SGML")]
    Topic__TextProcessing__Markup__SGML,
    #[strum(serialize = "Topic :: Text Processing :: Markup :: VRML")]
    Topic__TextProcessing__Markup__VRML,
    #[strum(serialize = "Topic :: Text Processing :: Markup :: XML")]
    Topic__TextProcessing__Markup__XML,
    #[strum(serialize = "Topic :: Text Processing :: Markup :: reStructuredText")]
    Topic__TextProcessing__Markup__reStructuredText,
    #[strum(serialize = "Topic :: Utilities")]
    Topic__Utilities,
    #[strum(serialize = "Typing :: Stubs Only")]
    Typing__StubsOnly,
    #[strum(serialize = "Typing :: Typed")]
    Typing__Typed,
}

impl Classifier {
    pub fn split(&self) -> Split<'_, &str> {
        self.as_ref().split(" :: ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn string_round_trip() {
        let trove = "Programming Language :: Rust";
        assert_eq!(Classifier::from_str(&trove).unwrap().as_ref(), trove);
    }

    #[test]
    fn split_round_trip() {
        let trove = Classifier::License__OSIApproved__GNUGeneralPublicLicensev3orlaterGPLv3plus;

        let vec_trove = trove.split().collect::<Vec<&str>>();
        assert_eq!(
            vec_trove,
            vec![
                "License",
                "OSI Approved",
                "GNU General Public License v3 or later (GPLv3+)"
            ]
        );

        let string_trove = vec_trove.join(" :: ");
        assert_eq!(
            string_trove,
            "License :: OSI Approved :: GNU General Public License v3 or later (GPLv3+)"
        );

        let new_trove = Classifier::from_str(&string_trove).unwrap();
        assert_eq!(new_trove, trove);
    }
}
