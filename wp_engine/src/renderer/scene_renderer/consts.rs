pub const VERTEX_SHADER: &'static str = r#"
#version 150

#define GLSL 1
#define HLSL 0
#define highp

#define CAST2(x) (vec2(x))
#define CAST3(x) (vec3(x))
#define CAST4(x) (vec4(x))
#define CAST3X3(x) (mat3(x))

#define texSample2D texture
#define texSample2DLod textureLod
#define mul(x, y) ((y) * (x))
#define frac fract
#define atan2 atan
#define fmod(x, y) (x-y*trunc(x/y))
#define ddx dFdx
#define ddy(x) dFdy(-(x))
#define saturate(x) (clamp(x, 0.0, 1.0))

#define max(x, y) max(y, x)

#define float1 float
#define float2 vec2
#define float3 vec3
#define float4 vec4
#define lerp mix

#define attribute in
#define varying out

#define M_PI 3.14159265359
#define M_PI_HALF 1.57079632679
#define M_PI_2 6.28318530718

#define SQRT_2 1.41421356237
#define SQRT_3 1.73205080756

vec3 hsv2rgb(vec3 c)
{
	vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
	vec3 p = abs(frac(c.xxx + K.xyz) * 6.0 - K.www);
	return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 rgb2hsv(vec3 RGB)
{
	vec4 P = (RGB.g < RGB.b) ? vec4(RGB.bg, -1.0, 2.0/3.0) : vec4(RGB.gb, 0.0, -1.0/3.0);
	vec4 Q = (RGB.r < P.x) ? vec4(P.xyw, RGB.r) : vec4(RGB.r, P.yzx);
	float C = Q.x - min(Q.w, Q.y);
	float H = abs((Q.w - Q.y) / (6.0 * C + 1e-10) + Q.z);

	vec3 HCV = vec3(H, C, Q.x);
	float S = HCV.y / (HCV.z + 1e-10);
	return vec3(HCV.x, S, HCV.z);
}

vec2 rotateVec2(vec2 v, float r)
{
	vec2 cs = vec2(cos(r), sin(r));
	return vec2(v.x * cs.x - v.y * cs.y, v.x * cs.y + v.y * cs.x);
}

float greyscale(vec3 color)
{
	return dot(color, vec3(0.11, 0.59, 0.3));
}

mat3 squareToQuad(vec2 p0, vec2 p1, vec2 p2, vec2 p3) {
	mat3 m = mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
	float dx0 = p0.x;
	float dy0 = p0.y;
	float dx1 = p1.x;
	float dy1 = p1.y;
	
	float dx2 = p3.x;
	float dy2 = p3.y;
	float dx3 = p2.x;
	float dy3 = p2.y;
	
	float diffx1 = dx1 - dx3;
	float diffy1 = dy1 - dy3;
	float diffx2 = dx2 - dx3;
	float diffy2 = dy2 - dy3;

	float det = diffx1*diffy2 - diffx2*diffy1;
	float sumx = dx0 - dx1 + dx3 - dx2;
	float sumy = dy0 - dy1 + dy3 - dy2;

	if (det == 0.0 || (sumx == 0.0 && sumy == 0.0)) {
		m[0][0] = dx1 - dx0;
		m[0][1] = dy1 - dy0;
		m[0][2] = 0.0;
		m[1][0] = dx3 - dx1;
		m[1][1] = dy3 - dy1;
		m[1][2] = 0.0;
		m[2][0] = dx0;
		m[2][1] = dy0;
		m[2][2] = 1.0;
		return m;
	} else {
		float ovdet = 1.0 / det;
		float g = (sumx * diffy2 - diffx2 * sumy) * ovdet;
		float h = (diffx1 * sumy - sumx * diffy1) * ovdet;

		m[0][0] = dx1 - dx0 + g * dx1;
		m[0][1] = dy1 - dy0 + g * dy1;
		m[0][2] = g;
		m[1][0] = dx2 - dx0 + h * dx2;
		m[1][1] = dy2 - dy0 + h * dy2;
		m[1][2] = h;
		m[2][0] = dx0;
		m[2][1] = dy0;
		m[2][2] = 1.0;
		return m;
	}
}

#if HLSL
mat3 inverse(mat3 m) {
	float a00 = m[0][0], a01 = m[0][1], a02 = m[0][2];
	float a10 = m[1][0], a11 = m[1][1], a12 = m[1][2];
	float a20 = m[2][0], a21 = m[2][1], a22 = m[2][2];
	float b01 = a22 * a11 - a12 * a21;
	float b11 = -a22 * a10 + a12 * a20;
	float b21 = a21 * a10 - a11 * a20;
	float det = a00 * b01 + a01 * b11 + a02 * b21;
	return mat3(b01, (-a22 * a01 + a02 * a21), (a12 * a01 - a02 * a11),
			  b11, (a22 * a00 - a02 * a20), (-a12 * a00 + a02 * a10),
			  b21, (-a21 * a00 + a01 * a20), (a11 * a00 - a01 * a10)) / det;
}
#endif

uniform mat4 g_ModelViewProjectionMatrix;
uniform float g_Time;
uniform vec4 g_Texture1Resolution;
uniform vec4 g_Texture2Resolution;
uniform float g_Direction; // {"material":"direction","label":"ui_editor_properties_direction","default":0,"direction":true}

attribute vec3 a_Position;
attribute vec2 a_TexCoord;

varying vec4 v_TexCoord;
varying vec2 v_Direction;

uniform vec2 g_Point0; // {"material":"point0","label":"p0","default":"0 0"}
uniform vec2 g_Point1; // {"material":"point1","label":"p1","default":"1 0"}
uniform vec2 g_Point2; // {"material":"point2","label":"p2","default":"1 1"}
uniform vec2 g_Point3; // {"material":"point3","label":"p3","default":"0 1"}

varying vec3 v_TexCoordPerspective;

void main() {
	gl_Position = mul(vec4(a_Position, 1.0), g_ModelViewProjectionMatrix);
	v_TexCoord = a_TexCoord.xyxy;

	v_TexCoord.z *= g_Texture1Resolution.z / g_Texture1Resolution.x;
	v_TexCoord.w *= g_Texture1Resolution.w / g_Texture1Resolution.y;

	v_Direction = rotateVec2(vec2(0, 1), g_Direction);

	mat3 xform = inverse(squareToQuad(g_Point0, g_Point1, g_Point2, g_Point3));
	v_TexCoordPerspective.xyz = mul(vec3(a_TexCoord.xy, 1.0), xform);
}
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
#version 150

#define GLSL 1
#define HLSL 0
#define highp

#define CAST2(x) (vec2(x))
#define CAST3(x) (vec3(x))
#define CAST4(x) (vec4(x))
#define CAST3X3(x) (mat3(x))

#define texSample2D texture
#define texSample2DLod textureLod
#define mul(x, y) ((y) * (x))
#define frac fract
#define atan2 atan
#define fmod(x, y) (x-y*trunc(x/y))
#define ddx dFdx
#define ddy(x) dFdy(-(x))
#define saturate(x) (clamp(x, 0.0, 1.0))

#define max(x, y) max(y, x)

#define float1 float
#define float2 vec2
#define float3 vec3
#define float4 vec4
#define lerp mix

#define varying in
#define gl_FragColor glOutColor
out vec4 glOutColor;


#define M_PI 3.14159265359
#define M_PI_HALF 1.57079632679
#define M_PI_2 6.28318530718

#define SQRT_2 1.41421356237
#define SQRT_3 1.73205080756

vec3 hsv2rgb(vec3 c)
{
	vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
	vec3 p = abs(frac(c.xxx + K.xyz) * 6.0 - K.www);
	return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 rgb2hsv(vec3 RGB)
{
	vec4 P = (RGB.g < RGB.b) ? vec4(RGB.bg, -1.0, 2.0/3.0) : vec4(RGB.gb, 0.0, -1.0/3.0);
	vec4 Q = (RGB.r < P.x) ? vec4(P.xyw, RGB.r) : vec4(RGB.r, P.yzx);
	float C = Q.x - min(Q.w, Q.y);
	float H = abs((Q.w - Q.y) / (6.0 * C + 1e-10) + Q.z);

	vec3 HCV = vec3(H, C, Q.x);
	float S = HCV.y / (HCV.z + 1e-10);
	return vec3(HCV.x, S, HCV.z);
}

vec2 rotateVec2(vec2 v, float r)
{
	vec2 cs = vec2(cos(r), sin(r));
	return vec2(v.x * cs.x - v.y * cs.y, v.x * cs.y + v.y * cs.x);
}

float greyscale(vec3 color)
{
	return dot(color, vec3(0.11, 0.59, 0.3));
}

varying vec4 v_TexCoord;
varying vec2 v_Direction;

varying vec3 v_TexCoordPerspective;

uniform sampler2D g_Texture0; // {"hidden":true}
uniform sampler2D g_Texture1; // {"label":"ui_editor_properties_opacity_mask","mode":"opacitymask","combo":"MASK","paintdefaultcolor":"0 0 0 1"}
uniform sampler2D g_Texture2; // {"label":"ui_editor_properties_time_offset","mode":"opacitymask","default":"util/black","combo":"TIMEOFFSET"}
uniform float g_Time;

uniform float g_Speed; // {"material":"speed","label":"ui_editor_properties_speed","default":5,"range":[0.01,50]}
uniform float g_Scale; // {"material":"scale","label":"ui_editor_properties_scale","default":200,"range":[0.01,1000]}
uniform float g_Strength; // {"material":"strength","label":"ui_editor_properties_strength","default":0.1,"range":[0.01,1]}
uniform float g_Perspective; // {"material":"perspective","label":"ui_editor_properties_perspective","default":0,"range":[0,0.2]}

void main() {
	float mask = texSample2D(g_Texture1, v_TexCoord.zw).r;

	vec2 texCoord = v_TexCoord.xy;
	vec2 texCoordMotion = texCoord;

	texCoordMotion = v_TexCoordPerspective.xy / v_TexCoordPerspective.z;

	float pos = abs(dot((texCoordMotion - 0.5), v_Direction));
	float distance = g_Time * g_Speed + dot(texCoordMotion, v_Direction) * (g_Scale + g_Perspective * pos);

	distance *= step(0.0, v_TexCoordPerspective.z);

	distance += texSample2D(g_Texture2, v_TexCoord.zw).r * M_PI_2;

	vec2 offset = vec2(v_Direction.y, -v_Direction.x);
	float strength = g_Strength * g_Strength + g_Perspective * pos;
	texCoord += sin(distance) * offset * strength * mask;

	gl_FragColor = texSample2D(g_Texture0, texCoord);
}
"#;

pub const VERTEX_SHADER_1: &'static str = r#"
#version 330

layout(location = 0) in vec3 a_Position;
layout(location = 1) in vec2 a_TexCoord;

out vec2 texCoord;

uniform mat4 g_ModelViewProjectionMatrix;

void main() {
	texCoord = a_TexCoord;
	gl_Position = vec4(a_Position, 1.0);
}
"#;

pub const FRAGMENT_SHADER_1: &'static str = r#"
#version 330

in vec2 texCoord;

out vec4 outColor;

void main() {
	// outColor = vec4(texCoord.x, 0.0, 0.0, 1.0);
	outColor = vec4(texCoord, 0.0, 1.0);
}
"#;

pub const VERTEX_SHADER_2: &'static str = r#"
#version 330

layout(location = 0) in vec3 a_Position;
layout(location = 1) in vec2 a_TexCoord;

out vec2 texCoord;

uniform mat4 g_ModelViewProjectionMatrix;

void main() {
	texCoord = a_TexCoord;
	gl_Position = g_ModelViewProjectionMatrix * vec4(a_Position, 1.0);
}
"#;

pub const FRAGMENT_SHADER_2: &'static str = r#"
#version 330

in vec2 texCoord;

out vec4 outColor;

uniform sampler2D g_Texture0;
uniform int g_Texture0Id;

void main() {
    outColor = 0.5 * texture(g_Texture0, texCoord);
}
  
"#;
