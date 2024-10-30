#
# spec file for package agama-yast
#
# Copyright (c) 2024 SUSE LLC
#
# All modifications and additions to the file contributed by third parties
# remain the property of their copyright owners, unless otherwise agreed
# upon. The license for this file, and modifications and additions to the
# file, is the same license as for the pristine package itself (unless the
# license for the pristine package is not an Open Source License, in which
# case the license is the MIT License). An "Open Source License" is a
# license that conforms to the Open Source Definition (Version 1.9)
# published by the Open Source Initiative.

# Please submit bugfixes or comments via https://bugs.opensuse.org/
#


Name:           agama-yast
Version:        10.devel54
Release:        0
%define mod_name agama-yast
%define mod_full_name %{mod_name}-%{version}
BuildRequires:  dbus-1-common
# "msgfmt" tool
BuildRequires:  gettext-runtime
Requires:       dbus-1-common

BuildRoot:      %{_tmppath}/%{name}-%{version}-build
BuildRequires:  ruby-macros >= 5

URL:            https://github.com/openSUSE/agama
Source:         %{mod_full_name}.gem
Source1:        po.tar.bz2
Source2:        install_translations.sh
Summary:        YaST integration service for Agama - common files
License:        GPL-2.0-only
Group:          Development/Languages/Ruby

%description
D-Bus service exposing some YaST features that are useful for Agama.

%prep
%{gem_unpack}

%build

%install
BASE=%{mod_full_name}
install -D -m 0644 ${BASE}/share/dbus.conf %{buildroot}%{_datadir}/dbus-1/agama.conf
install --directory %{buildroot}%{_datadir}/dbus-1/agama-services
install -m 0644 --target-directory=%{buildroot}%{_datadir}/dbus-1/agama-services ${BASE}/share/org.opensuse.Agama*.service
install -D -m 0644 ${BASE}/share/agama.service %{buildroot}%{_unitdir}/agama.service
install -D -m 0644 ${BASE}/share/agama-proxy-setup.service %{buildroot}%{_unitdir}/agama-proxy-setup.service
install --directory %{buildroot}/usr/share/agama/conf.d
install -D -m 0644 ${BASE}/conf.d/*.yaml %{buildroot}/usr/share/agama/conf.d/

# run a script for installing the translations
sh "%{SOURCE2}" "%{SOURCE1}"

%pre
%service_add_pre agama.service

%post
%service_add_post agama.service

%preun
%service_del_preun agama.service

%postun
%service_del_postun_with_restart agama.service

%files
%{_datadir}/dbus-1/agama.conf
%dir %{_datadir}/dbus-1/agama-services
%{_datadir}/dbus-1/agama-services/org.opensuse.Agama*.service
%{_unitdir}/agama.service
%{_unitdir}/agama-proxy-setup.service
%dir %{_datadir}/agama
%dir %{_datadir}/agama/conf.d
%{_datadir}/agama/conf.d
%dir /usr/share/YaST2
/usr/share/YaST2/locale

%changelog
