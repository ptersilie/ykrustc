use rustc_ast::ast;
use rustc_middle::mir::interpret::{
    truncate, Allocation, ConstValue, LitToConstError, LitToConstInput, Scalar,
};
use rustc_middle::ty::{self, ParamEnv, TyCtxt, TyS};
use rustc_span::symbol::Symbol;
use rustc_target::abi::Size;

crate fn lit_to_const<'tcx>(
    tcx: TyCtxt<'tcx>,
    lit_input: LitToConstInput<'tcx>,
) -> Result<&'tcx ty::Const<'tcx>, LitToConstError> {
    let LitToConstInput { lit, ty, neg } = lit_input;

    let trunc = |n| {
        let param_ty = ParamEnv::reveal_all().and(ty);
        let width = tcx.layout_of(param_ty).map_err(|_| LitToConstError::Reported)?.size;
        trace!("trunc {} with size {} and shift {}", n, width.bits(), 128 - width.bits());
        let result = truncate(n, width);
        trace!("trunc result: {}", result);
        Ok(ConstValue::Scalar(Scalar::from_uint(result, width)))
    };

    let lit = match (lit, &ty.kind) {
        (ast::LitKind::Str(s, _), ty::Ref(_, TyS { kind: ty::Str, .. }, _)) => {
            let s = s.as_str();
            let allocation = Allocation::from_byte_aligned_bytes(s.as_bytes());
            let allocation = tcx.intern_const_alloc(allocation);
            ConstValue::Slice { data: allocation, start: 0, end: s.len() }
        }
        (ast::LitKind::ByteStr(data), ty::Ref(_, TyS { kind: ty::Slice(_), .. }, _)) => {
            let allocation = Allocation::from_byte_aligned_bytes(data as &Vec<u8>);
            let allocation = tcx.intern_const_alloc(allocation);
            ConstValue::Slice { data: allocation, start: 0, end: data.len() }
        }
        (ast::LitKind::ByteStr(data), ty::Ref(_, TyS { kind: ty::Array(_, _), .. }, _)) => {
            let id = tcx.allocate_bytes(data);
            ConstValue::Scalar(Scalar::Ptr(id.into()))
        }
        (ast::LitKind::Byte(n), ty::Uint(ast::UintTy::U8)) => {
            ConstValue::Scalar(Scalar::from_uint(*n, Size::from_bytes(1)))
        }
        (ast::LitKind::Int(n, _), ty::Uint(_)) | (ast::LitKind::Int(n, _), ty::Int(_)) => {
            trunc(if neg { (*n as i128).overflowing_neg().0 as u128 } else { *n })?
        }
        (ast::LitKind::Float(n, _), ty::Float(fty)) => {
            parse_float(*n, *fty, neg).map_err(|_| LitToConstError::UnparseableFloat)?
        }
        (ast::LitKind::Bool(b), ty::Bool) => ConstValue::Scalar(Scalar::from_bool(*b)),
        (ast::LitKind::Char(c), ty::Char) => ConstValue::Scalar(Scalar::from_char(*c)),
        (ast::LitKind::Err(_), _) => return Err(LitToConstError::Reported),
        _ => return Err(LitToConstError::TypeError),
    };
    Ok(ty::Const::from_value(tcx, lit, ty))
}

fn parse_float<'tcx>(num: Symbol, fty: ast::FloatTy, neg: bool) -> Result<ConstValue<'tcx>, ()> {
    let num = num.as_str();
    use rustc_apfloat::ieee::{Double, Single};
    let scalar = match fty {
        ast::FloatTy::F32 => {
            num.parse::<f32>().map_err(|_| ())?;
            let mut f = num.parse::<Single>().unwrap_or_else(|e| {
                panic!("apfloat::ieee::Single failed to parse `{}`: {:?}", num, e)
            });
            if neg {
                f = -f;
            }
            Scalar::from_f32(f)
        }
        ast::FloatTy::F64 => {
            num.parse::<f64>().map_err(|_| ())?;
            let mut f = num.parse::<Double>().unwrap_or_else(|e| {
                panic!("apfloat::ieee::Double failed to parse `{}`: {:?}", num, e)
            });
            if neg {
                f = -f;
            }
            Scalar::from_f64(f)
        }
    };

    Ok(ConstValue::Scalar(scalar))
}
