from typing import Any, Final, final

@final
class AnimationIn: ...

@final
class AnimationInType:
    BlurIn: Final[AnimationInType]
    Bounce: Final[AnimationInType]
    Fade: Final[AnimationInType]
    FlipX: Final[AnimationInType]
    FlipY: Final[AnimationInType]
    Null: Final[AnimationInType]
    SlideDown: Final[AnimationInType]
    SlideLeft: Final[AnimationInType]
    SlideRight: Final[AnimationInType]
    SlideUp: Final[AnimationInType]
    Swing: Final[AnimationInType]
    ZoomIn: Final[AnimationInType]
    ZoomOut: Final[AnimationInType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class AnimationOut: ...

@final
class AnimationOutType:
    BlurOut: Final[AnimationOutType]
    Fade: Final[AnimationOutType]
    Null: Final[AnimationOutType]
    SlideDown: Final[AnimationOutType]
    SlideLeft: Final[AnimationOutType]
    SlideRight: Final[AnimationOutType]
    SlideUp: Final[AnimationOutType]
    ZoomIn: Final[AnimationOutType]
    ZoomOut: Final[AnimationOutType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class AnimationTarget:
    MediaOnly: Final[AnimationTarget]
    PerElement: Final[AnimationTarget]
    TextOnly: Final[AnimationTarget]
    WholeSegment: Final[AnimationTarget]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class AnnotationPosition:
    Above: Final[AnnotationPosition]
    Below: Final[AnnotationPosition]
    Left: Final[AnnotationPosition]
    Right: Final[AnnotationPosition]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

class AppearAt:
    @final
    class At(AppearAt):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> float: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: float) -> AppearAt.At: ...

    @final
    class End(AppearAt):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> AppearAt.End: ...

    @final
    class Start(AppearAt):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> AppearAt.Start: ...

@final
class AspectRatio:
    Classic: Final[AspectRatio]
    Square: Final[AspectRatio]
    Vertical: Final[AspectRatio]
    Wide: Final[AspectRatio]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class AssetRegistry: ...

@final
class AudioAsset: ...

@final
class AudioBlock: ...

@final
class AudioCodec:
    Aac: Final[AudioCodec]
    Mp3: Final[AudioCodec]
    Opus: Final[AudioCodec]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

class AxisPosition:
    @final
    class Named(AxisPosition):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> NamedPosition: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: NamedPosition) -> AxisPosition.Named: ...

    @final
    class Relative(AxisPosition):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> float: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: float) -> AxisPosition.Relative: ...

class Background:
    @final
    class Gradient(Background):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> GradientBackground: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: GradientBackground) -> Background.Gradient: ...

    @final
    class Image(Background):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> ImageBackground: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: ImageBackground) -> Background.Image: ...

    @final
    class Solid(Background):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> SolidBackground: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: SolidBackground) -> Background.Solid: ...

    @final
    class Video(Background):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> VideoBackground: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: VideoBackground) -> Background.Video: ...

@final
class BackgroundFit:
    Contain: Final[BackgroundFit]
    Cover: Final[BackgroundFit]
    Stretch: Final[BackgroundFit]
    Tile: Final[BackgroundFit]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class BackgroundMusic: ...

@final
class BackgroundOverlay: ...

@final
class BetterIs:
    Higher: Final[BetterIs]
    Lower: Final[BetterIs]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class BodyAnimation: ...

@final
class BodyAnimationType:
    Breathe: Final[BodyAnimationType]
    Float: Final[BodyAnimationType]
    Glitch: Final[BodyAnimationType]
    Null: Final[BodyAnimationType]
    Pulse: Final[BodyAnimationType]
    Shake: Final[BodyAnimationType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class Border: ...

@final
class BorderStyle:
    Dashed: Final[BorderStyle]
    Dotted: Final[BorderStyle]
    Solid: Final[BorderStyle]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class CaptionPosition:
    Above: Final[CaptionPosition]
    Below: Final[CaptionPosition]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class CellHighlight: ...

class CellValue:
    @final
    class Bool(CellValue):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> bool: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: bool) -> CellValue.Bool: ...

    @final
    class Null(CellValue):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> CellValue.Null: ...

    @final
    class Number(CellValue):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> float: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: float) -> CellValue.Number: ...

    @final
    class Text(CellValue):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> str: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: str) -> CellValue.Text: ...

@final
class CodeAnnotation: ...

@final
class CodeBlock: ...

@final
class CodeFont: ...

@final
class CodeLanguage:
    Bash: Final[CodeLanguage]
    C: Final[CodeLanguage]
    Cpp: Final[CodeLanguage]
    Go: Final[CodeLanguage]
    Java: Final[CodeLanguage]
    JavaScript: Final[CodeLanguage]
    Json: Final[CodeLanguage]
    Kotlin: Final[CodeLanguage]
    Markdown: Final[CodeLanguage]
    Plaintext: Final[CodeLanguage]
    Pseudocode: Final[CodeLanguage]
    Python: Final[CodeLanguage]
    Rust: Final[CodeLanguage]
    Sql: Final[CodeLanguage]
    Swift: Final[CodeLanguage]
    Toml: Final[CodeLanguage]
    TypeScript: Final[CodeLanguage]
    Yaml: Final[CodeLanguage]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class CodePart: ...

@final
class CodeRevealMode:
    AllAtOnce: Final[CodeRevealMode]
    BlockByBlock: Final[CodeRevealMode]
    LineByLine: Final[CodeRevealMode]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class CodeRevealSync:
    Audio: Final[CodeRevealSync]
    Timed: Final[CodeRevealSync]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class CodeSize: ...

@final
class CodeTheme:
    Dark: Final[CodeTheme]
    Dracula: Final[CodeTheme]
    GithubDark: Final[CodeTheme]
    Light: Final[CodeTheme]
    Monokai: Final[CodeTheme]
    Nord: Final[CodeTheme]
    OneDark: Final[CodeTheme]
    SolarizedDark: Final[CodeTheme]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class ColHighlight: ...

@final
class Color: ...

@final
class ColorPalette:
    Cool: Final[ColorPalette]
    MatchBackground: Final[ColorPalette]
    Monochrome: Final[ColorPalette]
    Muted: Final[ColorPalette]
    Vibrant: Final[ColorPalette]
    Warm: Final[ColorPalette]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class ColumnType:
    Badge: Final[ColumnType]
    Bar: Final[ColumnType]
    Boolean: Final[ColumnType]
    Currency: Final[ColumnType]
    Number: Final[ColumnType]
    Percentage: Final[ColumnType]
    Text: Final[ColumnType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class ComparisonConfig: ...

@final
class Easing:
    Bounce: Final[Easing]
    EaseIn: Final[Easing]
    EaseInOut: Final[Easing]
    EaseOut: Final[Easing]
    Linear: Final[Easing]
    Spring: Final[Easing]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class ElevenLabsConfig: ...

@final
class ElevenLabsModel:
    ElevenMonolingualV1: Final[ElevenLabsModel]
    ElevenMultilingualV2: Final[ElevenLabsModel]
    ElevenTurboV2: Final[ElevenLabsModel]
    ElevenTurboV25: Final[ElevenLabsModel]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class FillBackground: ...

@final
class FontAsset: ...

@final
class FontConfig: ...

@final
class FontStyle:
    Italic: Final[FontStyle]
    Normal: Final[FontStyle]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class FontWeight:
    Black: Final[FontWeight]
    Bold: Final[FontWeight]
    Light: Final[FontWeight]
    Medium: Final[FontWeight]
    Regular: Final[FontWeight]
    Semibold: Final[FontWeight]
    Thin: Final[FontWeight]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class GradientBackground: ...

@final
class GradientDef: ...

@final
class GradientDirection:
    DiagonalTl: Final[GradientDirection]
    DiagonalTr: Final[GradientDirection]
    Horizontal: Final[GradientDirection]
    Radial: Final[GradientDirection]
    Vertical: Final[GradientDirection]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class GradientStop: ...

@final
class ImageAsset: ...

@final
class ImageBackground: ...

@final
class ImageBlock: ...

@final
class ImageMood:
    Friendly: Final[ImageMood]
    Neutral: Final[ImageMood]
    Professional: Final[ImageMood]
    Serious: Final[ImageMood]
    Technical: Final[ImageMood]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class ImageStyle:
    Chart: Final[ImageStyle]
    Diagram: Final[ImageStyle]
    FlatIcon: Final[ImageStyle]
    Illustration: Final[ImageStyle]
    Infographic: Final[ImageStyle]
    Realistic: Final[ImageStyle]
    Render3D: Final[ImageStyle]
    Sketch: Final[ImageStyle]
    Whiteboard: Final[ImageStyle]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class KeyTerm: ...

@final
class Layout: ...

@final
class LayoutMode:
    Free: Final[LayoutMode]
    Grid: Final[LayoutMode]
    Overlay: Final[LayoutMode]
    SplitLeft: Final[LayoutMode]
    SplitRight: Final[LayoutMode]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

class LoopCount:
    @final
    class Count(LoopCount):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> int: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: int) -> LoopCount.Count: ...

    @final
    class Infinite(LoopCount):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> LoopCount.Infinite: ...

@final
class MathBlock: ...

@final
class MathRenderMode:
    Block: Final[MathRenderMode]
    Inline: Final[MathRenderMode]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

class MathRevealAt:
    @final
    class At(MathRevealAt):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> float: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: float) -> MathRevealAt.At: ...

    @final
    class End(MathRevealAt):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> MathRevealAt.End: ...

    @final
    class Start(MathRevealAt):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> MathRevealAt.Start: ...

    @final
    class WithWord(MathRevealAt):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> MathRevealAt.WithWord: ...

@final
class MathRevealStyle:
    Appear: Final[MathRevealStyle]
    Draw: Final[MathRevealStyle]
    Fade: Final[MathRevealStyle]
    ZoomIn: Final[MathRevealStyle]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class Mood:
    Analytical: Final[Mood]
    Excited: Final[Mood]
    Neutral: Final[Mood]
    Serious: Final[Mood]
    Tense: Final[Mood]
    Warm: Final[Mood]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class NamedPosition:
    Bottom: Final[NamedPosition]
    Center: Final[NamedPosition]
    Left: Final[NamedPosition]
    Right: Final[NamedPosition]
    Top: Final[NamedPosition]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class Opacity: ...

@final
class OutputConfig: ...

@final
class OutputFormat:
    Mov: Final[OutputFormat]
    Mp4: Final[OutputFormat]
    Webm: Final[OutputFormat]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class OutputProfile: ...

@final
class Pacing:
    Fast: Final[Pacing]
    Normal: Final[Pacing]
    Slow: Final[Pacing]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class Padding: ...

class PartAppearAt:
    @final
    class At(PartAppearAt):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> float: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: float) -> PartAppearAt.At: ...

    @final
    class Start(PartAppearAt):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> PartAppearAt.Start: ...

    @final
    class WithAudioCue(PartAppearAt):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> str: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: str) -> PartAppearAt.WithAudioCue: ...

    @final
    class WithPartId(PartAppearAt):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> str: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: str) -> PartAppearAt.WithPartId: ...

@final
class Placeholder: ...

@final
class PlaceholderIcon:
    Chart: Final[PlaceholderIcon]
    Diagram: Final[PlaceholderIcon]
    Image: Final[PlaceholderIcon]
    Null: Final[PlaceholderIcon]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class Position: ...

@final
class PreviousPartHighlight: ...

class RepeatMode:
    @final
    class Count(RepeatMode):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> int: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: int) -> RepeatMode.Count: ...

    @final
    class Loop(RepeatMode):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> RepeatMode.Loop: ...

    @final
    class Once(RepeatMode):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> RepeatMode.Once: ...

@final
class Resolution: ...

@final
class RevealSync:
    Audio: Final[RevealSync]
    Timed: Final[RevealSync]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class RowHighlight: ...

@final
class SafeZone: ...

@final
class Segment: ...

@final
class SegmentAnimation: ...

@final
class SegmentControl: ...

@final
class SegmentDefaults: ...

@final
class SegmentLoop: ...

@final
class SegmentPart: ...

@final
class SegmentType:
    Chapter: Final[SegmentType]
    Outro: Final[SegmentType]
    Pause: Final[SegmentType]
    Speech: Final[SegmentType]
    Title: Final[SegmentType]
    Transition: Final[SegmentType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class Shadow: ...

@final
class ShapeAnimation: ...

@final
class ShapeAnimationType:
    Draw: Final[ShapeAnimationType]
    Fade: Final[ShapeAnimationType]
    Grow: Final[ShapeAnimationType]
    Null: Final[ShapeAnimationType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class ShapeBlock: ...

@final
class ShapeFill: ...

@final
class ShapeStroke: ...

@final
class ShapeType:
    Arrow: Final[ShapeType]
    Circle: Final[ShapeType]
    Ellipse: Final[ShapeType]
    Line: Final[ShapeType]
    Polygon: Final[ShapeType]
    Rect: Final[ShapeType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class Size: ...

class SizeValue:
    @final
    class Auto(SizeValue):
        __match_args__: Final = ()
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /) -> SizeValue.Auto: ...

    @final
    class Fraction(SizeValue):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> float: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: float) -> SizeValue.Fraction: ...

    @final
    class Pixels(SizeValue):
        __match_args__: Final = ("_0",)
        @property
        def _0(self, /) -> int: ...
        def __getitem__(self, /, key: int) -> Any: ...
        def __len__(self, /) -> int: ...
        def __new__(cls, /, _0: int) -> SizeValue.Pixels: ...

@final
class SolidBackground: ...

@final
class SoundEffect: ...

@final
class Stagger: ...

@final
class StaggerOrder:
    CenterOut: Final[StaggerOrder]
    OutsideIn: Final[StaggerOrder]
    Random: Final[StaggerOrder]
    Sequential: Final[StaggerOrder]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class TableBlock: ...

@final
class TableCaption: ...

@final
class TableColors: ...

@final
class TableFont: ...

@final
class TableRevealMode:
    AllAtOnce: Final[TableRevealMode]
    ColByCol: Final[TableRevealMode]
    RowByRow: Final[TableRevealMode]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class TableStyle:
    Accounting: Final[TableStyle]
    Bordered: Final[TableStyle]
    Comparison: Final[TableStyle]
    DarkCard: Final[TableStyle]
    Minimal: Final[TableStyle]
    Striped: Final[TableStyle]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class TextAlign:
    Center: Final[TextAlign]
    Justify: Final[TextAlign]
    Left: Final[TextAlign]
    Right: Final[TextAlign]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class TextBlock: ...

@final
class TextRevealMode:
    AllAtOnce: Final[TextRevealMode]
    LineByLine: Final[TextRevealMode]
    Typewriter: Final[TextRevealMode]
    WordByWord: Final[TextRevealMode]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class TotalsRow: ...

@final
class Transition: ...

@final
class TransitionType:
    BlurThrough: Final[TransitionType]
    Crossfade: Final[TransitionType]
    Cut: Final[TransitionType]
    Fade: Final[TransitionType]
    Null: Final[TransitionType]
    WipeDown: Final[TransitionType]
    WipeLeft: Final[TransitionType]
    WipeRight: Final[TransitionType]
    WipeUp: Final[TransitionType]
    ZoomThrough: Final[TransitionType]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class TtsOutput: ...

@final
class VideoBackground: ...

@final
class VideoCodec:
    Av1: Final[VideoCodec]
    H264: Final[VideoCodec]
    H265: Final[VideoCodec]
    Vp9: Final[VideoCodec]
    def __int__(self, /) -> int: ...
    def __repr__(self, /) -> str: ...

@final
class VideoInput: ...

@final
class WordTimestamp: ...

def create_video(json_text: str, audio_temp_dir: str, output_path: str) -> str: ...
def version() -> str: ...
