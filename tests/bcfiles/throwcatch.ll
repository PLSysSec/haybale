; ModuleID = 'throwcatch.cpp'
source_filename = "throwcatch.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@_ZTIi = external constant i8*
@_ZTIh = external constant i8*

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @doesnt_throw(i32 %0) #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) !dbg !101 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i8, align 1
  %6 = alloca i8*, align 8
  %7 = alloca i32, align 4
  store i32 %0, i32* %3, align 4
  call void @llvm.dbg.declare(metadata i32* %3, metadata !104, metadata !DIExpression()), !dbg !105
  call void @llvm.dbg.declare(metadata i32* %4, metadata !106, metadata !DIExpression()), !dbg !108
  store volatile i32 0, i32* %4, align 4, !dbg !108
  call void @llvm.dbg.declare(metadata i8* %5, metadata !109, metadata !DIExpression()), !dbg !112
  store volatile i8 0, i8* %5, align 1, !dbg !112
  store volatile i32 10, i32* %4, align 4, !dbg !113
  %8 = load volatile i8, i8* %5, align 1, !dbg !115
  %9 = trunc i8 %8 to i1, !dbg !115
  br i1 %9, label %10, label %20, !dbg !117

10:                                               ; preds = %1
  %11 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !118
  %12 = bitcast i8* %11 to i32*, !dbg !118
  store i32 20, i32* %12, align 16, !dbg !118
  invoke void @__cxa_throw(i8* %11, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %46 unwind label %13, !dbg !118

13:                                               ; preds = %10
  %14 = landingpad { i8*, i32 }
          catch i8* null, !dbg !119
  %15 = extractvalue { i8*, i32 } %14, 0, !dbg !119
  store i8* %15, i8** %6, align 8, !dbg !119
  %16 = extractvalue { i8*, i32 } %14, 1, !dbg !119
  store i32 %16, i32* %7, align 4, !dbg !119
  br label %17, !dbg !119

17:                                               ; preds = %13
  %18 = load i8*, i8** %6, align 8, !dbg !120
  %19 = call i8* @__cxa_begin_catch(i8* %18) #4, !dbg !120
  store i32 -1, i32* %2, align 4, !dbg !121
  call void @__cxa_end_catch(), !dbg !123
  br label %44

20:                                               ; preds = %1
  br label %21, !dbg !120

21:                                               ; preds = %20
  %22 = load volatile i32, i32* %4, align 4, !dbg !124
  %23 = add nsw i32 %22, 1, !dbg !124
  store volatile i32 %23, i32* %4, align 4, !dbg !124
  %24 = load volatile i8, i8* %5, align 1, !dbg !125
  %25 = trunc i8 %24 to i1, !dbg !125
  br i1 %25, label %26, label %36, !dbg !128

26:                                               ; preds = %21
  %27 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !129
  %28 = bitcast i8* %27 to i32*, !dbg !129
  store i32 20, i32* %28, align 16, !dbg !129
  invoke void @__cxa_throw(i8* %27, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %46 unwind label %29, !dbg !129

29:                                               ; preds = %26
  %30 = landingpad { i8*, i32 }
          catch i8* null, !dbg !130
  %31 = extractvalue { i8*, i32 } %30, 0, !dbg !130
  store i8* %31, i8** %6, align 8, !dbg !130
  %32 = extractvalue { i8*, i32 } %30, 1, !dbg !130
  store i32 %32, i32* %7, align 4, !dbg !130
  br label %33, !dbg !130

33:                                               ; preds = %29
  %34 = load i8*, i8** %6, align 8, !dbg !131
  %35 = call i8* @__cxa_begin_catch(i8* %34) #4, !dbg !131
  store i32 -2, i32* %2, align 4, !dbg !132
  call void @__cxa_end_catch(), !dbg !134
  br label %44

36:                                               ; preds = %21
  %37 = load volatile i32, i32* %4, align 4, !dbg !135
  %38 = load i32, i32* %3, align 4, !dbg !137
  %39 = add nsw i32 %37, %38, !dbg !138
  %40 = icmp slt i32 %39, 100, !dbg !139
  br i1 %40, label %41, label %42, !dbg !140

41:                                               ; preds = %36
  store i32 1, i32* %2, align 4, !dbg !141
  br label %44, !dbg !141

42:                                               ; preds = %36
  store i32 2, i32* %2, align 4, !dbg !143
  br label %44, !dbg !143

43:                                               ; No predecessors!
  call void @llvm.trap(), !dbg !134
  unreachable, !dbg !134

44:                                               ; preds = %33, %42, %41, %17
  %45 = load i32, i32* %2, align 4, !dbg !145
  ret i32 %45, !dbg !145

46:                                               ; preds = %26, %10
  unreachable
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

declare i8* @__cxa_allocate_exception(i64)

declare void @__cxa_throw(i8*, i8*, i8*)

declare i32 @__gxx_personality_v0(...)

declare i8* @__cxa_begin_catch(i8*)

declare void @__cxa_end_catch()

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #2

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z14throw_uncaughti(i32 %0) #0 !dbg !146 {
  %2 = alloca i32, align 4
  store volatile i32 %0, i32* %2, align 4
  call void @llvm.dbg.declare(metadata i32* %2, metadata !149, metadata !DIExpression()), !dbg !150
  %3 = load volatile i32, i32* %2, align 4, !dbg !151
  %4 = srem i32 %3, 2, !dbg !153
  %5 = icmp ne i32 %4, 0, !dbg !151
  br i1 %5, label %6, label %7, !dbg !154

6:                                                ; preds = %1
  ret i32 2, !dbg !155

7:                                                ; preds = %1
  %8 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !157
  %9 = bitcast i8* %8 to i32*, !dbg !157
  store i32 20, i32* %9, align 16, !dbg !157
  call void @__cxa_throw(i8* %8, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5, !dbg !157
  unreachable, !dbg !157
}

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z21throw_multiple_valuesi(i32 %0) #0 !dbg !159 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  store volatile i32 %0, i32* %3, align 4
  call void @llvm.dbg.declare(metadata i32* %3, metadata !160, metadata !DIExpression()), !dbg !161
  %4 = load volatile i32, i32* %3, align 4, !dbg !162
  %5 = srem i32 %4, 4, !dbg !163
  switch i32 %5, label %11 [
    i32 1, label %6
    i32 2, label %7
    i32 3, label %8
  ], !dbg !164

6:                                                ; preds = %1
  store i32 1, i32* %2, align 4, !dbg !165
  br label %14, !dbg !165

7:                                                ; preds = %1
  store i32 2, i32* %2, align 4, !dbg !167
  br label %14, !dbg !167

8:                                                ; preds = %1
  %9 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !168
  %10 = bitcast i8* %9 to i32*, !dbg !168
  store i32 3, i32* %10, align 16, !dbg !168
  call void @__cxa_throw(i8* %9, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5, !dbg !168
  unreachable, !dbg !168

11:                                               ; preds = %1
  %12 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !169
  %13 = bitcast i8* %12 to i32*, !dbg !169
  store i32 4, i32* %13, align 16, !dbg !169
  call void @__cxa_throw(i8* %12, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5, !dbg !169
  unreachable, !dbg !169

14:                                               ; preds = %7, %6
  %15 = load i32, i32* %2, align 4, !dbg !170
  ret i32 %15, !dbg !170
}

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z24throw_uncaught_wrongtypei(i32 %0) #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) !dbg !171 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i8*, align 8
  %5 = alloca i32, align 4
  %6 = alloca i8, align 1
  store volatile i32 %0, i32* %3, align 4
  call void @llvm.dbg.declare(metadata i32* %3, metadata !172, metadata !DIExpression()), !dbg !173
  %7 = load volatile i32, i32* %3, align 4, !dbg !174
  %8 = srem i32 %7, 2, !dbg !177
  %9 = icmp ne i32 %8, 0, !dbg !174
  br i1 %9, label %10, label %11, !dbg !178

10:                                               ; preds = %1
  store i32 2, i32* %2, align 4, !dbg !179
  br label %27, !dbg !179

11:                                               ; preds = %1
  %12 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !181
  %13 = bitcast i8* %12 to i32*, !dbg !181
  store i32 20, i32* %13, align 16, !dbg !181
  invoke void @__cxa_throw(i8* %12, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %34 unwind label %14, !dbg !181

14:                                               ; preds = %11
  %15 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTIh to i8*), !dbg !183
  %16 = extractvalue { i8*, i32 } %15, 0, !dbg !183
  store i8* %16, i8** %4, align 8, !dbg !183
  %17 = extractvalue { i8*, i32 } %15, 1, !dbg !183
  store i32 %17, i32* %5, align 4, !dbg !183
  br label %18, !dbg !183

18:                                               ; preds = %14
  %19 = load i32, i32* %5, align 4, !dbg !184
  %20 = call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIh to i8*)) #4, !dbg !184
  %21 = icmp eq i32 %19, %20, !dbg !184
  br i1 %21, label %22, label %29, !dbg !184

22:                                               ; preds = %18
  call void @llvm.dbg.declare(metadata i8* %6, metadata !185, metadata !DIExpression()), !dbg !186
  %23 = load i8*, i8** %4, align 8, !dbg !184
  %24 = call i8* @__cxa_begin_catch(i8* %23) #4, !dbg !184
  %25 = load i8, i8* %24, align 1, !dbg !184
  store i8 %25, i8* %6, align 1, !dbg !184
  store i32 10, i32* %2, align 4, !dbg !187
  call void @__cxa_end_catch() #4, !dbg !189
  br label %27

26:                                               ; No predecessors!
  call void @llvm.trap(), !dbg !189
  unreachable, !dbg !189

27:                                               ; preds = %22, %10
  %28 = load i32, i32* %2, align 4, !dbg !190
  ret i32 %28, !dbg !190

29:                                               ; preds = %18
  %30 = load i8*, i8** %4, align 8, !dbg !184
  %31 = load i32, i32* %5, align 4, !dbg !184
  %32 = insertvalue { i8*, i32 } undef, i8* %30, 0, !dbg !184
  %33 = insertvalue { i8*, i32 } %32, i32 %31, 1, !dbg !184
  resume { i8*, i32 } %33, !dbg !184

34:                                               ; preds = %11
  unreachable
}

; Function Attrs: nounwind readnone
declare i32 @llvm.eh.typeid.for(i8*) #3

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local void @_Z19throw_uncaught_voidPVi(i32* %0) #0 !dbg !191 {
  %2 = alloca i32*, align 8
  store i32* %0, i32** %2, align 8
  call void @llvm.dbg.declare(metadata i32** %2, metadata !195, metadata !DIExpression()), !dbg !196
  %3 = load i32*, i32** %2, align 8, !dbg !197
  %4 = load volatile i32, i32* %3, align 4, !dbg !199
  %5 = icmp eq i32 %4, 0, !dbg !200
  br i1 %5, label %6, label %8, !dbg !201

6:                                                ; preds = %1
  %7 = load i32*, i32** %2, align 8, !dbg !202
  store volatile i32 1, i32* %7, align 4, !dbg !204
  br label %11, !dbg !205

8:                                                ; preds = %1
  %9 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !206
  %10 = bitcast i8* %9 to i32*, !dbg !206
  store i32 20, i32* %10, align 16, !dbg !206
  call void @__cxa_throw(i8* %9, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5, !dbg !206
  unreachable, !dbg !206

11:                                               ; preds = %6
  ret void, !dbg !208
}

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z21throw_uncaught_calleri(i32 %0) #0 !dbg !209 {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  store i32 %0, i32* %2, align 4
  call void @llvm.dbg.declare(metadata i32* %2, metadata !210, metadata !DIExpression()), !dbg !211
  call void @llvm.dbg.declare(metadata i32* %3, metadata !212, metadata !DIExpression()), !dbg !213
  %4 = load i32, i32* %2, align 4, !dbg !214
  store volatile i32 %4, i32* %3, align 4, !dbg !213
  call void @_Z19throw_uncaught_voidPVi(i32* %3), !dbg !215
  ret i32 1, !dbg !216
}

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z24throw_and_catch_wildcardb(i1 zeroext %0) #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) !dbg !217 {
  %2 = alloca i32, align 4
  %3 = alloca i8, align 1
  %4 = alloca i8*, align 8
  %5 = alloca i32, align 4
  %6 = zext i1 %0 to i8
  store i8 %6, i8* %3, align 1
  call void @llvm.dbg.declare(metadata i8* %3, metadata !220, metadata !DIExpression()), !dbg !221
  %7 = load i8, i8* %3, align 1, !dbg !222
  %8 = trunc i8 %7 to i1, !dbg !222
  br i1 %8, label %9, label %19, !dbg !225

9:                                                ; preds = %1
  %10 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !226
  %11 = bitcast i8* %10 to i32*, !dbg !226
  store i32 20, i32* %11, align 16, !dbg !226
  invoke void @__cxa_throw(i8* %10, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %23 unwind label %12, !dbg !226

12:                                               ; preds = %9
  %13 = landingpad { i8*, i32 }
          catch i8* null, !dbg !227
  %14 = extractvalue { i8*, i32 } %13, 0, !dbg !227
  store i8* %14, i8** %4, align 8, !dbg !227
  %15 = extractvalue { i8*, i32 } %13, 1, !dbg !227
  store i32 %15, i32* %5, align 4, !dbg !227
  br label %16, !dbg !227

16:                                               ; preds = %12
  %17 = load i8*, i8** %4, align 8, !dbg !228
  %18 = call i8* @__cxa_begin_catch(i8* %17) #4, !dbg !228
  store i32 5, i32* %2, align 4, !dbg !229
  call void @__cxa_end_catch(), !dbg !231
  br label %21

19:                                               ; preds = %1
  store i32 2, i32* %2, align 4, !dbg !232
  br label %21, !dbg !232

20:                                               ; No predecessors!
  call void @llvm.trap(), !dbg !231
  unreachable, !dbg !231

21:                                               ; preds = %16, %19
  %22 = load i32, i32* %2, align 4, !dbg !233
  ret i32 %22, !dbg !233

23:                                               ; preds = %9
  unreachable
}

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z19throw_and_catch_valb(i1 zeroext %0) #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) !dbg !234 {
  %2 = alloca i32, align 4
  %3 = alloca i8, align 1
  %4 = alloca i8*, align 8
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  %7 = zext i1 %0 to i8
  store i8 %7, i8* %3, align 1
  call void @llvm.dbg.declare(metadata i8* %3, metadata !235, metadata !DIExpression()), !dbg !236
  %8 = load i8, i8* %3, align 1, !dbg !237
  %9 = trunc i8 %8 to i1, !dbg !237
  br i1 %9, label %10, label %27, !dbg !240

10:                                               ; preds = %1
  %11 = call i8* @__cxa_allocate_exception(i64 4) #4, !dbg !241
  %12 = bitcast i8* %11 to i32*, !dbg !241
  store i32 20, i32* %12, align 16, !dbg !241
  invoke void @__cxa_throw(i8* %11, i8* bitcast (i8** @_ZTIi to i8*), i8* null) #5
          to label %36 unwind label %13, !dbg !241

13:                                               ; preds = %10
  %14 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTIi to i8*), !dbg !242
  %15 = extractvalue { i8*, i32 } %14, 0, !dbg !242
  store i8* %15, i8** %4, align 8, !dbg !242
  %16 = extractvalue { i8*, i32 } %14, 1, !dbg !242
  store i32 %16, i32* %5, align 4, !dbg !242
  br label %17, !dbg !242

17:                                               ; preds = %13
  %18 = load i32, i32* %5, align 4, !dbg !243
  %19 = call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIi to i8*)) #4, !dbg !243
  %20 = icmp eq i32 %18, %19, !dbg !243
  br i1 %20, label %21, label %31, !dbg !243

21:                                               ; preds = %17
  call void @llvm.dbg.declare(metadata i32* %6, metadata !244, metadata !DIExpression()), !dbg !245
  %22 = load i8*, i8** %4, align 8, !dbg !243
  %23 = call i8* @__cxa_begin_catch(i8* %22) #4, !dbg !243
  %24 = bitcast i8* %23 to i32*, !dbg !243
  %25 = load i32, i32* %24, align 4, !dbg !243
  store i32 %25, i32* %6, align 4, !dbg !243
  %26 = load i32, i32* %6, align 4, !dbg !246
  store i32 %26, i32* %2, align 4, !dbg !248
  call void @__cxa_end_catch() #4, !dbg !249
  br label %29

27:                                               ; preds = %1
  store i32 2, i32* %2, align 4, !dbg !250
  br label %29, !dbg !250

28:                                               ; No predecessors!
  call void @llvm.trap(), !dbg !249
  unreachable, !dbg !249

29:                                               ; preds = %21, %27
  %30 = load i32, i32* %2, align 4, !dbg !251
  ret i32 %30, !dbg !251

31:                                               ; preds = %17
  %32 = load i8*, i8** %4, align 8, !dbg !243
  %33 = load i32, i32* %5, align 4, !dbg !243
  %34 = insertvalue { i8*, i32 } undef, i8* %32, 0, !dbg !243
  %35 = insertvalue { i8*, i32 } %34, i32 %33, 1, !dbg !243
  resume { i8*, i32 } %35, !dbg !243

36:                                               ; preds = %10
  unreachable
}

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z25throw_and_catch_in_callerb(i1 zeroext %0) #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) !dbg !252 {
  %2 = alloca i32, align 4
  %3 = alloca i8, align 1
  %4 = alloca i32, align 4
  %5 = alloca i8*, align 8
  %6 = alloca i32, align 4
  %7 = alloca i32, align 4
  %8 = zext i1 %0 to i8
  store i8 %8, i8* %3, align 1
  call void @llvm.dbg.declare(metadata i8* %3, metadata !253, metadata !DIExpression()), !dbg !254
  call void @llvm.dbg.declare(metadata i32* %4, metadata !255, metadata !DIExpression()), !dbg !256
  store volatile i32 2, i32* %4, align 4, !dbg !256
  %9 = load i8, i8* %3, align 1, !dbg !257
  %10 = trunc i8 %9 to i1, !dbg !257
  br i1 %10, label %11, label %27, !dbg !260

11:                                               ; preds = %1
  invoke void @_Z19throw_uncaught_voidPVi(i32* %4)
          to label %12 unwind label %13, !dbg !261

12:                                               ; preds = %11
  br label %27, !dbg !261

13:                                               ; preds = %11
  %14 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTIi to i8*), !dbg !262
  %15 = extractvalue { i8*, i32 } %14, 0, !dbg !262
  store i8* %15, i8** %5, align 8, !dbg !262
  %16 = extractvalue { i8*, i32 } %14, 1, !dbg !262
  store i32 %16, i32* %6, align 4, !dbg !262
  br label %17, !dbg !262

17:                                               ; preds = %13
  %18 = load i32, i32* %6, align 4, !dbg !263
  %19 = call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIi to i8*)) #4, !dbg !263
  %20 = icmp eq i32 %18, %19, !dbg !263
  br i1 %20, label %21, label %31, !dbg !263

21:                                               ; preds = %17
  call void @llvm.dbg.declare(metadata i32* %7, metadata !264, metadata !DIExpression()), !dbg !265
  %22 = load i8*, i8** %5, align 8, !dbg !263
  %23 = call i8* @__cxa_begin_catch(i8* %22) #4, !dbg !263
  %24 = bitcast i8* %23 to i32*, !dbg !263
  %25 = load i32, i32* %24, align 4, !dbg !263
  store i32 %25, i32* %7, align 4, !dbg !263
  %26 = load i32, i32* %7, align 4, !dbg !266
  store i32 %26, i32* %2, align 4, !dbg !268
  call void @__cxa_end_catch() #4, !dbg !269
  br label %29

27:                                               ; preds = %12, %1
  br label %28, !dbg !263

28:                                               ; preds = %27
  store i32 2, i32* %2, align 4, !dbg !270
  br label %29, !dbg !270

29:                                               ; preds = %28, %21
  %30 = load i32, i32* %2, align 4, !dbg !271
  ret i32 %30, !dbg !271

31:                                               ; preds = %17
  %32 = load i8*, i8** %5, align 8, !dbg !263
  %33 = load i32, i32* %6, align 4, !dbg !263
  %34 = insertvalue { i8*, i32 } undef, i8* %32, 0, !dbg !263
  %35 = insertvalue { i8*, i32 } %34, i32 %33, 1, !dbg !263
  resume { i8*, i32 } %35, !dbg !263
}

; Function Attrs: mustprogress noinline optnone sspstrong uwtable
define dso_local i32 @_Z27throw_and_rethrow_in_callerb(i1 zeroext %0) #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) !dbg !272 {
  %2 = alloca i8, align 1
  %3 = alloca i32, align 4
  %4 = alloca i8*, align 8
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  %7 = zext i1 %0 to i8
  store i8 %7, i8* %2, align 1
  call void @llvm.dbg.declare(metadata i8* %2, metadata !273, metadata !DIExpression()), !dbg !274
  call void @llvm.dbg.declare(metadata i32* %3, metadata !275, metadata !DIExpression()), !dbg !276
  store volatile i32 2, i32* %3, align 4, !dbg !276
  %8 = load i8, i8* %2, align 1, !dbg !277
  %9 = trunc i8 %8 to i1, !dbg !277
  br i1 %9, label %10, label %25, !dbg !280

10:                                               ; preds = %1
  invoke void @_Z19throw_uncaught_voidPVi(i32* %3)
          to label %11 unwind label %12, !dbg !281

11:                                               ; preds = %10
  br label %25, !dbg !281

12:                                               ; preds = %10
  %13 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTIi to i8*), !dbg !282
  %14 = extractvalue { i8*, i32 } %13, 0, !dbg !282
  store i8* %14, i8** %4, align 8, !dbg !282
  %15 = extractvalue { i8*, i32 } %13, 1, !dbg !282
  store i32 %15, i32* %5, align 4, !dbg !282
  br label %16, !dbg !282

16:                                               ; preds = %12
  %17 = load i32, i32* %5, align 4, !dbg !283
  %18 = call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTIi to i8*)) #4, !dbg !283
  %19 = icmp eq i32 %17, %18, !dbg !283
  br i1 %19, label %20, label %31, !dbg !283

20:                                               ; preds = %16
  call void @llvm.dbg.declare(metadata i32* %6, metadata !284, metadata !DIExpression()), !dbg !285
  %21 = load i8*, i8** %4, align 8, !dbg !283
  %22 = call i8* @__cxa_begin_catch(i8* %21) #4, !dbg !283
  %23 = bitcast i8* %22 to i32*, !dbg !283
  %24 = load i32, i32* %23, align 4, !dbg !283
  store i32 %24, i32* %6, align 4, !dbg !283
  invoke void @__cxa_rethrow() #5
          to label %36 unwind label %26, !dbg !286

25:                                               ; preds = %11, %1
  br label %30, !dbg !283

26:                                               ; preds = %20
  %27 = landingpad { i8*, i32 }
          cleanup, !dbg !288
  %28 = extractvalue { i8*, i32 } %27, 0, !dbg !288
  store i8* %28, i8** %4, align 8, !dbg !288
  %29 = extractvalue { i8*, i32 } %27, 1, !dbg !288
  store i32 %29, i32* %5, align 4, !dbg !288
  call void @__cxa_end_catch() #4, !dbg !289
  br label %31, !dbg !289

30:                                               ; preds = %25
  ret i32 2, !dbg !290

31:                                               ; preds = %26, %16
  %32 = load i8*, i8** %4, align 8, !dbg !283
  %33 = load i32, i32* %5, align 4, !dbg !283
  %34 = insertvalue { i8*, i32 } undef, i8* %32, 0, !dbg !283
  %35 = insertvalue { i8*, i32 } %34, i32 %33, 1, !dbg !283
  resume { i8*, i32 } %35, !dbg !283

36:                                               ; preds = %20
  unreachable
}

declare void @__cxa_rethrow()

attributes #0 = { mustprogress noinline optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { cold noreturn nounwind }
attributes #3 = { nounwind readnone }
attributes #4 = { nounwind }
attributes #5 = { noreturn }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!93, !94, !95, !96, !97, !98, !99}
!llvm.ident = !{!100}

!0 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !1, producer: "clang version 13.0.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2, retainedTypes: !3, imports: !9, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "throwcatch.cpp", directory: "/home/notroot/smbshare/CS530/haybale/tests/bcfiles")
!2 = !{}
!3 = !{!4}
!4 = !DIDerivedType(tag: DW_TAG_typedef, name: "int32_t", file: !5, line: 26, baseType: !6)
!5 = !DIFile(filename: "/usr/include/bits/stdint-intn.h", directory: "")
!6 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int32_t", file: !7, line: 41, baseType: !8)
!7 = !DIFile(filename: "/usr/include/bits/types.h", directory: "")
!8 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!9 = !{!10, !16, !20, !21, !25, !28, !30, !32, !34, !37, !40, !43, !46, !49, !51, !56, !60, !64, !68, !70, !72, !74, !76, !79, !82, !85, !88, !91}
!10 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !12, file: !15, line: 47)
!11 = !DINamespace(name: "std", scope: null)
!12 = !DIDerivedType(tag: DW_TAG_typedef, name: "int8_t", file: !5, line: 24, baseType: !13)
!13 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int8_t", file: !7, line: 37, baseType: !14)
!14 = !DIBasicType(name: "signed char", size: 8, encoding: DW_ATE_signed_char)
!15 = !DIFile(filename: "/usr/bin/../lib64/gcc/x86_64-pc-linux-gnu/11.2.0/../../../../include/c++/11.2.0/cstdint", directory: "")
!16 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !17, file: !15, line: 48)
!17 = !DIDerivedType(tag: DW_TAG_typedef, name: "int16_t", file: !5, line: 25, baseType: !18)
!18 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int16_t", file: !7, line: 39, baseType: !19)
!19 = !DIBasicType(name: "short", size: 16, encoding: DW_ATE_signed)
!20 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !4, file: !15, line: 49)
!21 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !22, file: !15, line: 50)
!22 = !DIDerivedType(tag: DW_TAG_typedef, name: "int64_t", file: !5, line: 27, baseType: !23)
!23 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int64_t", file: !7, line: 44, baseType: !24)
!24 = !DIBasicType(name: "long int", size: 64, encoding: DW_ATE_signed)
!25 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !26, file: !15, line: 52)
!26 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast8_t", file: !27, line: 58, baseType: !14)
!27 = !DIFile(filename: "/usr/include/stdint.h", directory: "")
!28 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !29, file: !15, line: 53)
!29 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast16_t", file: !27, line: 60, baseType: !24)
!30 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !31, file: !15, line: 54)
!31 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast32_t", file: !27, line: 61, baseType: !24)
!32 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !33, file: !15, line: 55)
!33 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_fast64_t", file: !27, line: 62, baseType: !24)
!34 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !35, file: !15, line: 57)
!35 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least8_t", file: !27, line: 43, baseType: !36)
!36 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least8_t", file: !7, line: 52, baseType: !13)
!37 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !38, file: !15, line: 58)
!38 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least16_t", file: !27, line: 44, baseType: !39)
!39 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least16_t", file: !7, line: 54, baseType: !18)
!40 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !41, file: !15, line: 59)
!41 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least32_t", file: !27, line: 45, baseType: !42)
!42 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least32_t", file: !7, line: 56, baseType: !6)
!43 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !44, file: !15, line: 60)
!44 = !DIDerivedType(tag: DW_TAG_typedef, name: "int_least64_t", file: !27, line: 46, baseType: !45)
!45 = !DIDerivedType(tag: DW_TAG_typedef, name: "__int_least64_t", file: !7, line: 58, baseType: !23)
!46 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !47, file: !15, line: 62)
!47 = !DIDerivedType(tag: DW_TAG_typedef, name: "intmax_t", file: !27, line: 101, baseType: !48)
!48 = !DIDerivedType(tag: DW_TAG_typedef, name: "__intmax_t", file: !7, line: 72, baseType: !24)
!49 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !50, file: !15, line: 63)
!50 = !DIDerivedType(tag: DW_TAG_typedef, name: "intptr_t", file: !27, line: 87, baseType: !24)
!51 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !52, file: !15, line: 65)
!52 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint8_t", file: !53, line: 24, baseType: !54)
!53 = !DIFile(filename: "/usr/include/bits/stdint-uintn.h", directory: "")
!54 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint8_t", file: !7, line: 38, baseType: !55)
!55 = !DIBasicType(name: "unsigned char", size: 8, encoding: DW_ATE_unsigned_char)
!56 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !57, file: !15, line: 66)
!57 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint16_t", file: !53, line: 25, baseType: !58)
!58 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint16_t", file: !7, line: 40, baseType: !59)
!59 = !DIBasicType(name: "unsigned short", size: 16, encoding: DW_ATE_unsigned)
!60 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !61, file: !15, line: 67)
!61 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint32_t", file: !53, line: 26, baseType: !62)
!62 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint32_t", file: !7, line: 42, baseType: !63)
!63 = !DIBasicType(name: "unsigned int", size: 32, encoding: DW_ATE_unsigned)
!64 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !65, file: !15, line: 68)
!65 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint64_t", file: !53, line: 27, baseType: !66)
!66 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint64_t", file: !7, line: 45, baseType: !67)
!67 = !DIBasicType(name: "long unsigned int", size: 64, encoding: DW_ATE_unsigned)
!68 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !69, file: !15, line: 70)
!69 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast8_t", file: !27, line: 71, baseType: !55)
!70 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !71, file: !15, line: 71)
!71 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast16_t", file: !27, line: 73, baseType: !67)
!72 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !73, file: !15, line: 72)
!73 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast32_t", file: !27, line: 74, baseType: !67)
!74 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !75, file: !15, line: 73)
!75 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_fast64_t", file: !27, line: 75, baseType: !67)
!76 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !77, file: !15, line: 75)
!77 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least8_t", file: !27, line: 49, baseType: !78)
!78 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least8_t", file: !7, line: 53, baseType: !54)
!79 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !80, file: !15, line: 76)
!80 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least16_t", file: !27, line: 50, baseType: !81)
!81 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least16_t", file: !7, line: 55, baseType: !58)
!82 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !83, file: !15, line: 77)
!83 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least32_t", file: !27, line: 51, baseType: !84)
!84 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least32_t", file: !7, line: 57, baseType: !62)
!85 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !86, file: !15, line: 78)
!86 = !DIDerivedType(tag: DW_TAG_typedef, name: "uint_least64_t", file: !27, line: 52, baseType: !87)
!87 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uint_least64_t", file: !7, line: 59, baseType: !66)
!88 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !89, file: !15, line: 80)
!89 = !DIDerivedType(tag: DW_TAG_typedef, name: "uintmax_t", file: !27, line: 102, baseType: !90)
!90 = !DIDerivedType(tag: DW_TAG_typedef, name: "__uintmax_t", file: !7, line: 73, baseType: !67)
!91 = !DIImportedEntity(tag: DW_TAG_imported_declaration, scope: !11, entity: !92, file: !15, line: 81)
!92 = !DIDerivedType(tag: DW_TAG_typedef, name: "uintptr_t", file: !27, line: 90, baseType: !67)
!93 = !{i32 7, !"Dwarf Version", i32 4}
!94 = !{i32 2, !"Debug Info Version", i32 3}
!95 = !{i32 1, !"wchar_size", i32 4}
!96 = !{i32 7, !"PIC Level", i32 2}
!97 = !{i32 7, !"PIE Level", i32 2}
!98 = !{i32 7, !"uwtable", i32 1}
!99 = !{i32 7, !"frame-pointer", i32 2}
!100 = !{!"clang version 13.0.1"}
!101 = distinct !DISubprogram(name: "doesnt_throw", scope: !1, file: !1, line: 7, type: !102, scopeLine: 7, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!102 = !DISubroutineType(types: !103)
!103 = !{!8, !8}
!104 = !DILocalVariable(name: "a", arg: 1, scope: !101, file: !1, line: 7, type: !8)
!105 = !DILocation(line: 7, column: 22, scope: !101)
!106 = !DILocalVariable(name: "x", scope: !101, file: !1, line: 8, type: !107)
!107 = !DIDerivedType(tag: DW_TAG_volatile_type, baseType: !8)
!108 = !DILocation(line: 8, column: 18, scope: !101)
!109 = !DILocalVariable(name: "b", scope: !101, file: !1, line: 9, type: !110)
!110 = !DIDerivedType(tag: DW_TAG_volatile_type, baseType: !111)
!111 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!112 = !DILocation(line: 9, column: 19, scope: !101)
!113 = !DILocation(line: 11, column: 11, scope: !114)
!114 = distinct !DILexicalBlock(scope: !101, file: !1, line: 10, column: 9)
!115 = !DILocation(line: 12, column: 12, scope: !116)
!116 = distinct !DILexicalBlock(scope: !114, file: !1, line: 12, column: 12)
!117 = !DILocation(line: 12, column: 12, scope: !114)
!118 = !DILocation(line: 12, column: 15, scope: !116)
!119 = !DILocation(line: 27, column: 1, scope: !116)
!120 = !DILocation(line: 13, column: 5, scope: !114)
!121 = !DILocation(line: 14, column: 9, scope: !122)
!122 = distinct !DILexicalBlock(scope: !101, file: !1, line: 13, column: 19)
!123 = !DILocation(line: 15, column: 5, scope: !122)
!124 = !DILocation(line: 16, column: 6, scope: !101)
!125 = !DILocation(line: 18, column: 12, scope: !126)
!126 = distinct !DILexicalBlock(scope: !127, file: !1, line: 18, column: 12)
!127 = distinct !DILexicalBlock(scope: !101, file: !1, line: 17, column: 9)
!128 = !DILocation(line: 18, column: 12, scope: !127)
!129 = !DILocation(line: 18, column: 15, scope: !126)
!130 = !DILocation(line: 27, column: 1, scope: !126)
!131 = !DILocation(line: 24, column: 5, scope: !127)
!132 = !DILocation(line: 25, column: 9, scope: !133)
!133 = distinct !DILexicalBlock(scope: !101, file: !1, line: 24, column: 19)
!134 = !DILocation(line: 26, column: 5, scope: !133)
!135 = !DILocation(line: 19, column: 13, scope: !136)
!136 = distinct !DILexicalBlock(scope: !127, file: !1, line: 19, column: 13)
!137 = !DILocation(line: 19, column: 17, scope: !136)
!138 = !DILocation(line: 19, column: 15, scope: !136)
!139 = !DILocation(line: 19, column: 19, scope: !136)
!140 = !DILocation(line: 19, column: 13, scope: !127)
!141 = !DILocation(line: 20, column: 13, scope: !142)
!142 = distinct !DILexicalBlock(scope: !136, file: !1, line: 19, column: 26)
!143 = !DILocation(line: 22, column: 13, scope: !144)
!144 = distinct !DILexicalBlock(scope: !136, file: !1, line: 21, column: 16)
!145 = !DILocation(line: 27, column: 1, scope: !101)
!146 = distinct !DISubprogram(name: "throw_uncaught", linkageName: "_Z14throw_uncaughti", scope: !1, file: !1, line: 33, type: !147, scopeLine: 33, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!147 = !DISubroutineType(types: !148)
!148 = !{!8, !107}
!149 = !DILocalVariable(name: "a", arg: 1, scope: !146, file: !1, line: 33, type: !107)
!150 = !DILocation(line: 33, column: 33, scope: !146)
!151 = !DILocation(line: 34, column: 9, scope: !152)
!152 = distinct !DILexicalBlock(scope: !146, file: !1, line: 34, column: 9)
!153 = !DILocation(line: 34, column: 11, scope: !152)
!154 = !DILocation(line: 34, column: 9, scope: !146)
!155 = !DILocation(line: 35, column: 9, scope: !156)
!156 = distinct !DILexicalBlock(scope: !152, file: !1, line: 34, column: 16)
!157 = !DILocation(line: 37, column: 9, scope: !158)
!158 = distinct !DILexicalBlock(scope: !152, file: !1, line: 36, column: 12)
!159 = distinct !DISubprogram(name: "throw_multiple_values", linkageName: "_Z21throw_multiple_valuesi", scope: !1, file: !1, line: 42, type: !147, scopeLine: 42, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!160 = !DILocalVariable(name: "a", arg: 1, scope: !159, file: !1, line: 42, type: !107)
!161 = !DILocation(line: 42, column: 40, scope: !159)
!162 = !DILocation(line: 43, column: 13, scope: !159)
!163 = !DILocation(line: 43, column: 15, scope: !159)
!164 = !DILocation(line: 43, column: 5, scope: !159)
!165 = !DILocation(line: 44, column: 17, scope: !166)
!166 = distinct !DILexicalBlock(scope: !159, file: !1, line: 43, column: 20)
!167 = !DILocation(line: 45, column: 17, scope: !166)
!168 = !DILocation(line: 46, column: 17, scope: !166)
!169 = !DILocation(line: 47, column: 18, scope: !166)
!170 = !DILocation(line: 49, column: 1, scope: !159)
!171 = distinct !DISubprogram(name: "throw_uncaught_wrongtype", linkageName: "_Z24throw_uncaught_wrongtypei", scope: !1, file: !1, line: 52, type: !147, scopeLine: 52, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!172 = !DILocalVariable(name: "a", arg: 1, scope: !171, file: !1, line: 52, type: !107)
!173 = !DILocation(line: 52, column: 43, scope: !171)
!174 = !DILocation(line: 54, column: 13, scope: !175)
!175 = distinct !DILexicalBlock(scope: !176, file: !1, line: 54, column: 13)
!176 = distinct !DILexicalBlock(scope: !171, file: !1, line: 53, column: 9)
!177 = !DILocation(line: 54, column: 15, scope: !175)
!178 = !DILocation(line: 54, column: 13, scope: !176)
!179 = !DILocation(line: 55, column: 13, scope: !180)
!180 = distinct !DILexicalBlock(scope: !175, file: !1, line: 54, column: 20)
!181 = !DILocation(line: 57, column: 13, scope: !182)
!182 = distinct !DILexicalBlock(scope: !175, file: !1, line: 56, column: 16)
!183 = !DILocation(line: 62, column: 1, scope: !182)
!184 = !DILocation(line: 59, column: 5, scope: !176)
!185 = !DILocalVariable(name: "c", scope: !171, file: !1, line: 59, type: !55)
!186 = !DILocation(line: 59, column: 28, scope: !171)
!187 = !DILocation(line: 60, column: 9, scope: !188)
!188 = distinct !DILexicalBlock(scope: !171, file: !1, line: 59, column: 31)
!189 = !DILocation(line: 61, column: 5, scope: !188)
!190 = !DILocation(line: 62, column: 1, scope: !171)
!191 = distinct !DISubprogram(name: "throw_uncaught_void", linkageName: "_Z19throw_uncaught_voidPVi", scope: !1, file: !1, line: 65, type: !192, scopeLine: 65, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!192 = !DISubroutineType(types: !193)
!193 = !{null, !194}
!194 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !107, size: 64)
!195 = !DILocalVariable(name: "a", arg: 1, scope: !191, file: !1, line: 65, type: !194)
!196 = !DILocation(line: 65, column: 66, scope: !191)
!197 = !DILocation(line: 66, column: 10, scope: !198)
!198 = distinct !DILexicalBlock(scope: !191, file: !1, line: 66, column: 9)
!199 = !DILocation(line: 66, column: 9, scope: !198)
!200 = !DILocation(line: 66, column: 12, scope: !198)
!201 = !DILocation(line: 66, column: 9, scope: !191)
!202 = !DILocation(line: 67, column: 10, scope: !203)
!203 = distinct !DILexicalBlock(scope: !198, file: !1, line: 66, column: 18)
!204 = !DILocation(line: 67, column: 12, scope: !203)
!205 = !DILocation(line: 68, column: 5, scope: !203)
!206 = !DILocation(line: 69, column: 9, scope: !207)
!207 = distinct !DILexicalBlock(scope: !198, file: !1, line: 68, column: 12)
!208 = !DILocation(line: 71, column: 1, scope: !191)
!209 = distinct !DISubprogram(name: "throw_uncaught_caller", linkageName: "_Z21throw_uncaught_calleri", scope: !1, file: !1, line: 74, type: !102, scopeLine: 74, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!210 = !DILocalVariable(name: "a", arg: 1, scope: !209, file: !1, line: 74, type: !8)
!211 = !DILocation(line: 74, column: 31, scope: !209)
!212 = !DILocalVariable(name: "x", scope: !209, file: !1, line: 75, type: !107)
!213 = !DILocation(line: 75, column: 18, scope: !209)
!214 = !DILocation(line: 75, column: 22, scope: !209)
!215 = !DILocation(line: 76, column: 5, scope: !209)
!216 = !DILocation(line: 77, column: 5, scope: !209)
!217 = distinct !DISubprogram(name: "throw_and_catch_wildcard", linkageName: "_Z24throw_and_catch_wildcardb", scope: !1, file: !1, line: 81, type: !218, scopeLine: 81, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!218 = !DISubroutineType(types: !219)
!219 = !{!8, !111}
!220 = !DILocalVariable(name: "shouldthrow", arg: 1, scope: !217, file: !1, line: 81, type: !111)
!221 = !DILocation(line: 81, column: 35, scope: !217)
!222 = !DILocation(line: 83, column: 12, scope: !223)
!223 = distinct !DILexicalBlock(scope: !224, file: !1, line: 83, column: 12)
!224 = distinct !DILexicalBlock(scope: !217, file: !1, line: 82, column: 9)
!225 = !DILocation(line: 83, column: 12, scope: !224)
!226 = !DILocation(line: 83, column: 25, scope: !223)
!227 = !DILocation(line: 88, column: 1, scope: !223)
!228 = !DILocation(line: 85, column: 5, scope: !224)
!229 = !DILocation(line: 86, column: 9, scope: !230)
!230 = distinct !DILexicalBlock(scope: !217, file: !1, line: 85, column: 19)
!231 = !DILocation(line: 87, column: 5, scope: !230)
!232 = !DILocation(line: 84, column: 9, scope: !224)
!233 = !DILocation(line: 88, column: 1, scope: !217)
!234 = distinct !DISubprogram(name: "throw_and_catch_val", linkageName: "_Z19throw_and_catch_valb", scope: !1, file: !1, line: 91, type: !218, scopeLine: 91, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!235 = !DILocalVariable(name: "shouldthrow", arg: 1, scope: !234, file: !1, line: 91, type: !111)
!236 = !DILocation(line: 91, column: 30, scope: !234)
!237 = !DILocation(line: 93, column: 12, scope: !238)
!238 = distinct !DILexicalBlock(scope: !239, file: !1, line: 93, column: 12)
!239 = distinct !DILexicalBlock(scope: !234, file: !1, line: 92, column: 9)
!240 = !DILocation(line: 93, column: 12, scope: !239)
!241 = !DILocation(line: 93, column: 25, scope: !238)
!242 = !DILocation(line: 98, column: 1, scope: !238)
!243 = !DILocation(line: 95, column: 5, scope: !239)
!244 = !DILocalVariable(name: "e", scope: !234, file: !1, line: 95, type: !8)
!245 = !DILocation(line: 95, column: 18, scope: !234)
!246 = !DILocation(line: 96, column: 16, scope: !247)
!247 = distinct !DILexicalBlock(scope: !234, file: !1, line: 95, column: 21)
!248 = !DILocation(line: 96, column: 9, scope: !247)
!249 = !DILocation(line: 97, column: 5, scope: !247)
!250 = !DILocation(line: 94, column: 9, scope: !239)
!251 = !DILocation(line: 98, column: 1, scope: !234)
!252 = distinct !DISubprogram(name: "throw_and_catch_in_caller", linkageName: "_Z25throw_and_catch_in_callerb", scope: !1, file: !1, line: 101, type: !218, scopeLine: 101, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!253 = !DILocalVariable(name: "shouldthrow", arg: 1, scope: !252, file: !1, line: 101, type: !111)
!254 = !DILocation(line: 101, column: 36, scope: !252)
!255 = !DILocalVariable(name: "x", scope: !252, file: !1, line: 102, type: !107)
!256 = !DILocation(line: 102, column: 18, scope: !252)
!257 = !DILocation(line: 104, column: 12, scope: !258)
!258 = distinct !DILexicalBlock(scope: !259, file: !1, line: 104, column: 12)
!259 = distinct !DILexicalBlock(scope: !252, file: !1, line: 103, column: 9)
!260 = !DILocation(line: 104, column: 12, scope: !259)
!261 = !DILocation(line: 104, column: 25, scope: !258)
!262 = !DILocation(line: 109, column: 1, scope: !258)
!263 = !DILocation(line: 105, column: 5, scope: !259)
!264 = !DILocalVariable(name: "e", scope: !252, file: !1, line: 105, type: !8)
!265 = !DILocation(line: 105, column: 18, scope: !252)
!266 = !DILocation(line: 106, column: 16, scope: !267)
!267 = distinct !DILexicalBlock(scope: !252, file: !1, line: 105, column: 21)
!268 = !DILocation(line: 106, column: 9, scope: !267)
!269 = !DILocation(line: 107, column: 5, scope: !267)
!270 = !DILocation(line: 108, column: 5, scope: !252)
!271 = !DILocation(line: 109, column: 1, scope: !252)
!272 = distinct !DISubprogram(name: "throw_and_rethrow_in_caller", linkageName: "_Z27throw_and_rethrow_in_callerb", scope: !1, file: !1, line: 112, type: !218, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, retainedNodes: !2)
!273 = !DILocalVariable(name: "shouldthrow", arg: 1, scope: !272, file: !1, line: 112, type: !111)
!274 = !DILocation(line: 112, column: 38, scope: !272)
!275 = !DILocalVariable(name: "x", scope: !272, file: !1, line: 113, type: !107)
!276 = !DILocation(line: 113, column: 18, scope: !272)
!277 = !DILocation(line: 115, column: 12, scope: !278)
!278 = distinct !DILexicalBlock(scope: !279, file: !1, line: 115, column: 12)
!279 = distinct !DILexicalBlock(scope: !272, file: !1, line: 114, column: 9)
!280 = !DILocation(line: 115, column: 12, scope: !279)
!281 = !DILocation(line: 115, column: 25, scope: !278)
!282 = !DILocation(line: 121, column: 1, scope: !278)
!283 = !DILocation(line: 116, column: 5, scope: !279)
!284 = !DILocalVariable(name: "e", scope: !272, file: !1, line: 117, type: !8)
!285 = !DILocation(line: 117, column: 16, scope: !272)
!286 = !DILocation(line: 118, column: 9, scope: !287)
!287 = distinct !DILexicalBlock(scope: !272, file: !1, line: 117, column: 19)
!288 = !DILocation(line: 121, column: 1, scope: !287)
!289 = !DILocation(line: 119, column: 5, scope: !287)
!290 = !DILocation(line: 120, column: 5, scope: !272)
