#version 300 es
// Color screen based on on-screen-position

precision mediump float;
in vec4 screen_pos;
out vec4 FragColor;


uniform vec2 iResolution;
uniform float iTime;


// Created by inigo quilez - iq/2019
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.

// Basically the same as https://www.shadertoy.com/view/XlVcWz
// but optimized through symmetry so it only needs to evaluate
// four gears instead of 18. Also I made the gears with actual
// boxes rather than displacements, which creates an exact SDF
// allowing me to raymarch the scene at the speed of light, or
// in other words, without reducing the raymarching step size.
// Also I'm using a bounding volume to speed things up further
// so I can affor some nice ligthing and motion blur.
//
// Live streamed tutorial on this shader:
// PART 1: https://www.youtube.com/watch?v=sl9x19EnKng
// PART 2: https://www.youtube.com/watch?v=bdICU2uvOdU
//
// Video capture here: https://www.youtube.com/watch?v=ydTVmDBSGYQ
//
#define AA 1


// http://iquilezles.org/www/articles/smin/smin.htm
float smax( float a, float b, float k )
{
    float h = max(k-abs(a-b),0.0);
    return max(a, b) + h*h*0.25/k;
}

// http://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
float sdSphere( in vec3 p, in float r )
{
    return length(p)-r;
}

float sdVerticalSemiCapsule( vec3 p, float h, float r )
{
    p.y = max(p.y-h,0.0);
    return length( p ) - r;
}

// http://iquilezles.org/www/articles/distfunctions2d/distfunctions2d.htm
float sdCross( in vec2 p, in vec2 b, float r ) 
{
    p = abs(p); p = (p.y>p.x) ? p.yx : p.xy;
    
	vec2  q = p - b;
    float k = max(q.y,q.x);
    vec2  w = (k>0.0) ? q : vec2(b.y-p.x,-k);
    
    return sign(k)*length(max(w,0.0)) + r;
}

// https://www.shadertoy.com/view/MlycD3
float dot2( in vec2 v ) { return dot(v,v); }
float sdTrapezoid( in vec2 p, in float r1, float r2, float he )
{
    vec2 k1 = vec2(r2,he);
    vec2 k2 = vec2(r2-r1,2.0*he);

	p.x = abs(p.x);
    vec2 ca = vec2(max(0.0,p.x-((p.y<0.0)?r1:r2)), abs(p.y)-he);
    vec2 cb = p - k1 + k2*clamp( dot(k1-p,k2)/dot2(k2), 0.0, 1.0 );
    
    float s = (cb.x < 0.0 && ca.y < 0.0) ? -1.0 : 1.0;
    
    return s*sqrt( min(dot2(ca),dot2(cb)) );
}

// http://iquilezles.org/www/articles/intersectors/intersectors.htm
vec2 iSphere( in vec3 ro, in vec3 rd, in float rad )
{
	float b = dot( ro, rd );
	float c = dot( ro, ro ) - rad*rad;
	float h = b*b - c;
	if( h<0.0 ) return vec2(-1.0);
    h = sqrt(h);
	return vec2(-b-h, -b+h );
}

//----------------------------------

float dents( in vec2 q, in float tr, in float y )
{
    const float an = 6.283185/12.0;
    float fa = (atan(q.y,q.x)+an*0.5)/an;
    float sym = an*floor(fa);
    vec2 r = mat2(cos(sym),-sin(sym), sin(sym), cos(sym))*q;
    
#if 1
    float d = length(max(abs(r-vec2(0.17,0))-tr*vec2(0.042,0.041*y),0.0));
#else
    float d = sdTrapezoid( r.yx-vec2(0.0,0.17), 0.085*y, 0.028*y, tr*0.045 );
#endif

	return d - 0.005*tr;
}

vec4 gear(vec3 q, float off, float time)
{
    {
    float an = 2.0*time*sign(q.y) + off*6.283185/24.0;
    float co = cos(an), si = sin(an);
    q.xz = mat2(co,-si,si,co)*q.xz;
    }
    
    q.y = abs(q.y);
    
    float an2 = 2.0*min(1.0-2.0*abs(fract(0.5+time/10.0)-0.5),1.0/2.0);
    vec3 tr = min( 10.0*an2 - vec3(4.0,6.0,8.0),1.0);
    
    // ring
    float d = abs(length(q.xz) - 0.155*tr.y) - 0.018;

    // add dents
    float r = length(q);
    d = min( d, dents(q.xz,tr.z, r) );

    
    // slice it
    float de = -0.0015*clamp(600.0*abs(dot(q.xz,q.xz)-0.155*0.155),0.0,1.0);
    d = smax( d, abs(r-0.5)-0.03+de, 0.005*tr.z );

    // add cross
    float d3 = sdCross( q.xz, vec2(0.15,0.022)*tr.y, 0.02*tr.y );
    vec2 w = vec2( d3, abs(q.y-0.485)-0.005*tr.y );
    d3 = min(max(w.x,w.y),0.0) + length(max(w,0.0))-0.003*tr.y;
    d = min( d, d3 ); 
        
    // add pivot
    d = min( d, sdVerticalSemiCapsule( q, 0.5*tr.x, 0.01 ));

    // base
    d = min( d, sdSphere(q-vec3(0.0,0.12,0.0),0.025) );
    
    return vec4(d,q.xzy);
}

vec2 rot( vec2 v )
{
    return vec2(v.x-v.y,v.y+v.x)*0.707107;
}
    
vec4 map( in vec3 p, float time )
{
    // center sphere
    vec4 d = vec4( sdSphere(p,0.12), p );
    
    // gears. There are 18, but we only evaluate 4    
    vec3 qx = vec3(rot(p.zy),p.x); if(abs(qx.x)>abs(qx.y)) qx=qx.zxy;
    vec3 qy = vec3(rot(p.xz),p.y); if(abs(qy.x)>abs(qy.y)) qy=qy.zxy;
    vec3 qz = vec3(rot(p.yx),p.z); if(abs(qz.x)>abs(qz.y)) qz=qz.zxy;
    vec3 qa = abs(p); qa = (qa.x>qa.y && qa.x>qa.z) ? p.zxy : 
                           (qa.z>qa.y             ) ? p.yzx :
                                                      p.xyz;
    vec4 t;
    t = gear( qa,0.0,time ); if( t.x<d.x ) d=t;
    t = gear( qx,1.0,time ); if( t.x<d.x ) d=t;
    t = gear( qz,1.0,time ); if( t.x<d.x ) d=t;
    t = gear( qy,1.0,time ); if( t.x<d.x ) d=t;
    
	return d;
}

#define ZERO 0

// http://iquilezles.org/www/articles/normalsSDF/normalsSDF.htm
vec3 calcNormal( in vec3 pos, in float time )
{
#if 0
    vec2 e = vec2(1.0,-1.0)*0.5773;
    const float eps = 0.00025;
    return normalize( e.xyy*map( pos + e.xyy*eps, time ).x + 
					  e.yyx*map( pos + e.yyx*eps, time ).x + 
					  e.yxy*map( pos + e.yxy*eps, time ).x + 
					  e.xxx*map( pos + e.xxx*eps, time ).x );
#else
    // klems's trick to prevent the compiler from inlining map() 4 times
    vec3 n = vec3(0.0);
    for( int i=ZERO; i<4; i++ )
    {
        vec3 e = 0.5773*(2.0*vec3((((i+3)>>1)&1),((i>>1)&1),(i&1))-1.0);
        n += e*map(pos+0.0005*e,time).x;
    }
    return normalize(n);
#endif    
}

float calcAO( in vec3 pos, in vec3 nor, in float time )
{
	float occ = 0.0;
    float sca = 1.0;
    for( int i=ZERO; i<5; i++ )
    {
        float h = 0.01 + 0.12*float(i)/4.0;
        float d = map( pos+h*nor, time ).x;
        occ += (h-d)*sca;
        sca *= 0.95;
    }
    return clamp( 1.0 - 3.0*occ, 0.0, 1.0 );
}

// http://iquilezles.org/www/articles/rmshadows/rmshadows.htm
float calcSoftshadow( in vec3 ro, in vec3 rd, in float k, in float time )
{
    float res = 1.0;
    
    // bounding sphere
    vec2 b = iSphere( ro, rd, 0.535 );
	if( b.y>0.0 )
    {
        // raymarch
        float tmax = b.y;
        float t    = max(b.x,0.001);
        for( int i=0; i<64; i++ )
        {
            float h = map( ro + rd*t, time ).x;
            res = min( res, k*h/t );
            t += clamp( h, 0.012, 0.2 );
            if( res<0.001 || t>tmax ) break;
        }
    }
    
    return clamp( res, 0.0, 1.0 );
}

vec4 intersect( in vec3 ro, in vec3 rd, in float time )
{
    vec4 res = vec4(-1.0);
    
    // bounding sphere
    vec2 tminmax = iSphere( ro, rd, 0.535 );
	if( tminmax.y>0.0 )
    {
        // raymarch
        float t = max(tminmax.x,0.001);
        for( int i=0; i<128 && t<tminmax.y; i++ )
        {
            vec4 h = map(ro+t*rd,time);
            if( h.x<0.001 ) { res=vec4(t,h.yzw); break; }
            t += h.x;
        }
    }
    
    return res;
}

mat3 setCamera( in vec3 ro, in vec3 ta, float cr )
{
	vec3 cw = normalize(ta-ro);
	vec3 cp = vec3(sin(cr), cos(cr),0.0);
	vec3 cu = normalize( cross(cw,cp) );
	vec3 cv =          ( cross(cu,cw) );
    return mat3( cu, cv, cw );
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec3 tot = vec3(0.0);
    
    #if AA>1
    for( int m=ZERO; m<AA; m++ )
    for( int n=ZERO; n<AA; n++ )
    {
        // pixel coordinates
        vec2 o = vec2(float(m),float(n)) / float(AA) - 0.5;
        vec2 p = (2.0*(fragCoord+o)-iResolution.xy)/iResolution.y;
        float d = 0.5*sin(fragCoord.x*147.0)*sin(fragCoord.y*131.0);
        float time = iTime - 0.5*(1.0/24.0)*(float(m*AA+n)+d)/float(AA*AA-1);
        #else    
        vec2 p = (2.0*fragCoord-iResolution.xy)/iResolution.y;
        float time = iTime;
        #endif

	    // camera	
        float an = 6.2831*time/40.0;
        vec3 ta = vec3( 0.0, 0.0, 0.0 );
        vec3 ro = ta + vec3( 1.3*cos(an), 0.5, 1.2*sin(an) );
        
        ro += 0.005*sin(92.0*time/40.0+vec3(0.0,1.0,3.0));
        ta += 0.009*sin(68.0*time/40.0+vec3(2.0,4.0,6.0));
        
        // camera-to-world transformation
        mat3 ca = setCamera( ro, ta, 0.0 );
        
        // ray direction
        float fl = 2.0;
        vec3 rd = ca * normalize( vec3(p,fl) );

        // background
        vec3 col = vec3(1.0+rd.y)*0.03;
        
        // raymarch geometry
        vec4 tuvw = intersect( ro, rd, time );
        if( tuvw.x>0.0 )
        {
            // shading/lighting	
            vec3 pos = ro + tuvw.x*rd;
            vec3 nor = calcNormal(pos, time);
                        
            vec3 te = vec3(0.5);
            
            vec3 mate = 0.22*te;
            float len = length(pos);
            
            mate *= 1.0 + vec3(2.0,0.5,0.0)*(1.0-smoothstep(0.121,0.122,len) ) ;
            
            float focc  = 0.1+0.9*clamp(0.5+0.5*dot(nor,pos/len),0.0,1.0);
                  focc *= 0.1+0.9*clamp(len*2.0,0.0,1.0);
            float ks = clamp(te.x*1.5,0.0,1.0);
            vec3  f0 = mate;
            float kd = (1.0-ks)*0.125;
            
            float occ = calcAO( pos, nor, time ) * focc;
            
            col = vec3(0.0);
            
            // side
            {
            vec3  lig = normalize(vec3(0.8,0.2,0.6));
            float dif = clamp( dot(nor,lig), 0.0, 1.0 );
            vec3  hal = normalize(lig-rd);
            float sha = 1.0; if( dif>0.001 ) sha = calcSoftshadow( pos+0.001*nor, lig, 20.0, time );
            vec3  spe = pow(clamp(dot(nor,hal),0.0,1.0),16.0)*(f0+(1.0-f0)*pow(clamp(1.0+dot(hal,rd),0.0,1.0),5.0));
            col += kd*mate*2.0*vec3(1.00,0.70,0.50)*dif*sha;
            col += ks*     2.0*vec3(1.00,0.80,0.70)*dif*sha*spe*3.14;
            }

            // top
            {
            vec3  ref = reflect(rd,nor);
            float fre = clamp(1.0+dot(nor,rd),0.0,1.0);
            float sha = occ;
            col += kd*mate*25.0*vec3(0.19,0.22,0.24)*(0.6 + 0.4*nor.y)*sha;
            col += ks*     25.0*vec3(0.19,0.22,0.24)*sha*smoothstep( -1.0+1.5*focc, 1.0-0.4*focc, ref.y ) * (f0 + (1.0-f0)*pow(fre,5.0));
            }
            
            // bottom
            {
            float dif = clamp(0.4-0.6*nor.y,0.0,1.0);
            col += kd*mate*5.0*vec3(0.25,0.20,0.15)*dif*occ;
            }
        }
        
        // compress        
        // col = 1.2*col/(1.0+col);
        
        // vignetting
        col *= 1.0-0.1*dot(p,p);
        
        // gamma        
	    tot += pow(col,vec3(0.45) );
    #if AA>1
    }
    tot /= float(AA*AA);
    #endif

    // s-curve    
    tot = min(tot,1.0);
    tot = tot*tot*(3.0-2.0*tot);
    
    // cheap dithering
    tot += sin(fragCoord.x*114.0)*sin(fragCoord.y*211.1)/512.0;

    fragColor = vec4( tot, 1.0 );
}


void main() {
       mainImage(FragColor, (screen_pos.xy + 1.0) * 0.5 * iResolution);
}

