struct IKsPropertySet;
struct IBaseFilter;

namespace DShow {

bool IsVendorVideoHDR(IKsPropertySet *)
{
	return false;
}

void SetVendorVideoFormat(IKsPropertySet *, bool) {}

void SetVendorTonemapperUsage(IBaseFilter *, bool) {}

}
